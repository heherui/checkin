use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

use gtk4::prelude::*;
use gtk4::{Align, AspectFrame, Box as GtkBox, Button, GestureClick, Grid, Label, Widget};

use crate::core::{AppMode, AttendanceBook, AttendanceStatistics, Position, Table};
use crate::ui::cell_edit_dialog::{CellEditDialog, CellEditDraft};
use crate::ui::corner_add_button::CornerAddButton;
use crate::ui::status_dialog::StatusDialog;
use crate::ui::table_cell::TableCell;

const CELL_WIDTH_HEIGHT_RATIO: f32 = 2.0;

const CLASS_GRID: &str = "table-grid";
const CLASS_SELECTED: &str = "selected";
const CLASS_BOARD: &str = "table-board";

type StatusChangedCallback = Rc<dyn Fn(AttendanceStatistics) + 'static>;
type TableExportedCallback = Rc<dyn Fn(Table) + 'static>;

#[derive(Clone)]
struct CellWidgets {
    position: Position,
    container: GtkBox,
    surface: Label,
}

struct ViewState {
    mode: AppMode,
    selected_surface: Option<Label>,
    board: Option<AspectFrame>,
    table: Table,
    attendance: AttendanceBook,
    cells: Vec<CellWidgets>,
    row_action_buttons: Vec<Widget>,
    column_action_buttons: Vec<Widget>,
    on_status_change: Vec<StatusChangedCallback>,
    on_table_exported: Vec<TableExportedCallback>,
}

impl ViewState {
    fn new(table: &Table) -> Self {
        let table = table.clone();
        let attendance = AttendanceBook::new(&table);

        Self {
            mode: AppMode::default(),
            selected_surface: None,
            board: None,
            table,
            attendance,
            cells: Vec::new(),
            row_action_buttons: Vec::new(),
            column_action_buttons: Vec::new(),
            on_status_change: Vec::new(),
            on_table_exported: Vec::new(),
        }
    }
}

pub struct TableView {
    root: AspectFrame,
    state: Rc<RefCell<ViewState>>,
}

impl TableView {
    pub fn new(table: &Table) -> Self {
        let state = Rc::new(RefCell::new(ViewState::new(table)));
        let root = AspectFrame::builder()
            .ratio(Self::table_ratio(table))
            .hexpand(true)
            .vexpand(true)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .obey_child(false)
            .build();
        root.add_css_class(CLASS_BOARD);
        root.set_xalign(0.5);
        root.set_yalign(0.5);

        {
            let mut view_state = state.borrow_mut();
            view_state.board = Some(root.clone());
        }

        let grid = Self::build_grid(table, Rc::clone(&state));
        root.set_child(Some(&grid));

        Self { root, state }
    }

    pub fn connect_status_changed<F>(&self, callback: F)
    where
        F: Fn(AttendanceStatistics) + 'static,
    {
        let callback: StatusChangedCallback = Rc::new(callback);
        self.state.borrow_mut().on_status_change.push(callback);
    }

    pub fn connect_table_exported<F>(&self, callback: F)
    where
        F: Fn(Table) + 'static,
    {
        let callback: TableExportedCallback = Rc::new(callback);
        self.state.borrow_mut().on_table_exported.push(callback);
    }

    pub fn widget(&self) -> &AspectFrame {
        &self.root
    }

    pub fn set_mode(&self, mode: AppMode) {
        let mut statistics_to_emit = None;
        let mut table_to_emit: Option<(Table, Vec<TableExportedCallback>)> = None;

        {
            let Ok(mut state) = self.state.try_borrow_mut() else {
                return;
            };
            let previous_mode = state.mode;
            if previous_mode == mode {
                return;
            }
            state.mode = mode;

            if mode != AppMode::Edit {
                if let Some(previous) = state.selected_surface.take() {
                    previous.remove_css_class(CLASS_SELECTED);
                }
            }

            if previous_mode == AppMode::Edit && mode != AppMode::Edit {
                let table = state.table.clone();
                state.attendance.reconcile_with_table(&table);
                statistics_to_emit = Some(state.attendance.statistics(&table));
                table_to_emit = Some((table, state.on_table_exported.clone()));
            }
        }

        Self::render_all_cells(&self.state);

        if let Some(statistics) = statistics_to_emit {
            Self::emit_status_changed(&self.state, statistics);
        }

        if let Some((table, callbacks)) = table_to_emit {
            for callback in callbacks {
                callback(table.clone());
            }
        }
    }

    pub fn get_statistics(&self) -> AttendanceStatistics {
        let state = self.state.borrow();
        state.attendance.statistics(&state.table)
    }

    pub fn build_statistics_export_text_zh(&self, time: &SystemTime) -> String {
        let state = self.state.borrow();
        state.attendance.build_export_text_zh(&state.table, time)
    }

    fn table_ratio(table: &Table) -> f32 {
        let row_count = table.row_count().max(1) as f32;
        let column_count = table.column_count().max(1) as f32;
        CELL_WIDTH_HEIGHT_RATIO * (column_count / row_count)
    }

    fn build_grid(table: &Table, state: Rc<RefCell<ViewState>>) -> Grid {
        let grid = Grid::new();
        grid.set_row_spacing(1);
        grid.set_column_spacing(1);
        grid.set_row_homogeneous(false);
        grid.set_column_homogeneous(false);
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_halign(Align::Fill);
        grid.set_valign(Align::Fill);
        grid.add_css_class(CLASS_GRID);

        {
            let mut view_state = state.borrow_mut();
            view_state.cells.clear();
            view_state.row_action_buttons.clear();
            view_state.column_action_buttons.clear();
        }

        for y in 0..table.row_count() {
            for x in 0..table.column_count() {
                let position = Position { x, y };
                let cell = TableCell::new();
                Self::connect_cell_events(
                    cell.container(),
                    cell.surface(),
                    position,
                    Rc::clone(&state),
                );
                grid.attach(cell.container(), x as i32 + 1, y as i32 + 1, 1, 1);

                state.borrow_mut().cells.push(CellWidgets {
                    position,
                    container: cell.container().clone(),
                    surface: cell.surface().clone(),
                });
            }
        }

        for y in 0..table.row_count() {
            let button = Self::build_action_button("−");
            Self::connect_remove_row(&button, y, Rc::clone(&state));
            grid.attach(&button, 0, y as i32 + 1, 1, 1);
            state.borrow_mut().row_action_buttons.push(button.upcast());
        }

        for x in 0..table.column_count() {
            let button = Self::build_action_button("−");
            Self::connect_remove_column(&button, x, Rc::clone(&state));
            grid.attach(&button, x as i32 + 1, 0, 1, 1);
            state
                .borrow_mut()
                .column_action_buttons
                .push(button.upcast());
        }

        let corner_button = CornerAddButton::new();
        corner_button.connect_split(
            {
                let state = Rc::clone(&state);
                move || {
                    let statistics = {
                        let Ok(mut view_state) = state.try_borrow_mut() else {
                            return;
                        };
                        if view_state.mode != AppMode::Edit {
                            None
                        } else {
                            view_state.table.add_row();
                            let table = view_state.table.clone();
                            view_state.attendance.reconcile_with_table(&table);
                            Some(view_state.attendance.statistics(&table))
                        }
                    };
                    if let Some(statistics) = statistics {
                        Self::rebuild_grid(&state);
                        Self::emit_status_changed(&state, statistics);
                    }
                }
            },
            {
                let state = Rc::clone(&state);
                move || {
                    let statistics = {
                        let Ok(mut view_state) = state.try_borrow_mut() else {
                            return;
                        };
                        if view_state.mode != AppMode::Edit {
                            None
                        } else {
                            view_state.table.add_column();
                            let table = view_state.table.clone();
                            view_state.attendance.reconcile_with_table(&table);
                            Some(view_state.attendance.statistics(&table))
                        }
                    };
                    if let Some(statistics) = statistics {
                        Self::rebuild_grid(&state);
                        Self::emit_status_changed(&state, statistics);
                    }
                }
            },
        );
        grid.attach(corner_button.widget(), 0, 0, 1, 1);
        {
            let mut view_state = state.borrow_mut();
            view_state
                .row_action_buttons
                .push(corner_button.widget().clone().upcast());
            view_state
                .column_action_buttons
                .push(corner_button.widget().clone().upcast());
        }

        Self::render_all_cells(&state);
        grid
    }

    fn connect_cell_events(
        cell: &GtkBox,
        surface: &Label,
        position: Position,
        state: Rc<RefCell<ViewState>>,
    ) {
        let click = GestureClick::new();
        let weak_cell = cell.downgrade();
        let weak_surface = surface.downgrade();

        click.connect_pressed(move |_, n_press, _, _| {
            let Some(cell) = weak_cell.upgrade() else {
                return;
            };
            let Some(surface) = weak_surface.upgrade() else {
                return;
            };

            let mode = {
                let Ok(view_state) = state.try_borrow() else {
                    return;
                };
                view_state.mode
            };

            match mode {
                AppMode::Edit => {
                    if n_press == 1 {
                        Self::select_for_edit(&surface, &state);
                    }
                    if n_press == 2 {
                        Self::open_edit_dialog(&cell, position, Rc::clone(&state));
                    }
                }
                AppMode::CheckIn => {
                    if n_press != 2 {
                        return;
                    }
                    let can_checkin = {
                        let Ok(view_state) = state.try_borrow() else {
                            return;
                        };
                        !view_state.table.is_inert(position)
                    };
                    if can_checkin {
                        Self::open_status_dialog(&cell, &surface, position, Rc::clone(&state));
                    }
                }
            }
        });

        cell.add_controller(click);
    }

    fn open_status_dialog(
        cell: &GtkBox,
        surface: &Label,
        position: Position,
        state: Rc<RefCell<ViewState>>,
    ) {
        StatusDialog::present(cell, surface, move |status, _| {
            let statistics = {
                let Ok(mut view_state) = state.try_borrow_mut() else {
                    return;
                };
                let table = view_state.table.clone();
                if !view_state
                    .attendance
                    .update_status(&table, position, status)
                {
                    None
                } else {
                    Some(view_state.attendance.statistics(&table))
                }
            };

            if let Some(statistics) = statistics {
                Self::render_all_cells(&state);
                Self::emit_status_changed(&state, statistics);
            }
        });
    }

    fn open_edit_dialog(cell: &GtkBox, position: Position, state: Rc<RefCell<ViewState>>) {
        let initial = {
            let Ok(view_state) = state.try_borrow() else {
                return;
            };
            CellEditDraft::from_subject(view_state.table.subject_at(position))
        };

        CellEditDialog::present(cell, initial, move |draft| {
            let statistics = {
                let Ok(mut view_state) = state.try_borrow_mut() else {
                    return;
                };
                if !view_state.table.set_subject(position, draft.into_subject()) {
                    None
                } else {
                    let table = view_state.table.clone();
                    view_state.attendance.reconcile_with_table(&table);
                    Some(view_state.attendance.statistics(&table))
                }
            };

            if let Some(statistics) = statistics {
                Self::render_all_cells(&state);
                Self::emit_status_changed(&state, statistics);
            }
        });
    }

    fn select_for_edit(surface: &Label, state: &Rc<RefCell<ViewState>>) {
        let Ok(mut view_state) = state.try_borrow_mut() else {
            return;
        };
        if let Some(previous) = view_state.selected_surface.take() {
            previous.remove_css_class(CLASS_SELECTED);
        }
        surface.add_css_class(CLASS_SELECTED);
        view_state.selected_surface = Some(surface.clone());
    }

    fn connect_remove_row(button: &Button, row_index: u32, state: Rc<RefCell<ViewState>>) {
        button.connect_clicked(move |_| {
            let statistics = {
                let Ok(mut view_state) = state.try_borrow_mut() else {
                    return;
                };
                if view_state.mode != AppMode::Edit || !view_state.table.remove_row(row_index) {
                    None
                } else {
                    let table = view_state.table.clone();
                    view_state.attendance.reconcile_with_table(&table);
                    Some(view_state.attendance.statistics(&table))
                }
            };
            if let Some(statistics) = statistics {
                Self::rebuild_grid(&state);
                Self::emit_status_changed(&state, statistics);
            }
        });
    }

    fn connect_remove_column(button: &Button, column_index: u32, state: Rc<RefCell<ViewState>>) {
        button.connect_clicked(move |_| {
            let statistics = {
                let Ok(mut view_state) = state.try_borrow_mut() else {
                    return;
                };
                if view_state.mode != AppMode::Edit || !view_state.table.remove_column(column_index)
                {
                    None
                } else {
                    let table = view_state.table.clone();
                    view_state.attendance.reconcile_with_table(&table);
                    Some(view_state.attendance.statistics(&table))
                }
            };
            if let Some(statistics) = statistics {
                Self::rebuild_grid(&state);
                Self::emit_status_changed(&state, statistics);
            }
        });
    }

    fn rebuild_grid(state: &Rc<RefCell<ViewState>>) {
        let (table, board) = {
            let view_state = state.borrow();
            (view_state.table.clone(), view_state.board.clone())
        };
        let Some(board) = board else {
            return;
        };

        {
            let mut view_state = state.borrow_mut();
            if let Some(previous) = view_state.selected_surface.take() {
                previous.remove_css_class(CLASS_SELECTED);
            }
        }

        let grid = Self::build_grid(&table, Rc::clone(state));
        board.set_ratio(Self::table_ratio(&table));
        board.set_child(Some(&grid));
    }

    fn build_action_button(label: &str) -> Button {
        let button = Button::with_label(label);
        button.set_halign(Align::Fill);
        button.set_valign(Align::Fill);
        button.set_size_request(38, 30);
        button
    }

    fn render_all_cells(state: &Rc<RefCell<ViewState>>) {
        let (render_items, row_buttons, column_buttons, mode) = {
            let view_state = state.borrow();
            (
                view_state
                    .cells
                    .iter()
                    .map(|cell| {
                        let subject = view_state.table.subject_at_owned(cell.position);
                        let status = view_state.attendance.status_at(cell.position);
                        (
                            cell.container.clone(),
                            cell.surface.clone(),
                            view_state.mode,
                            subject,
                            status,
                        )
                    })
                    .collect::<Vec<_>>(),
                view_state.row_action_buttons.clone(),
                view_state.column_action_buttons.clone(),
                view_state.mode,
            )
        };

        for (container, surface, mode, subject, status) in render_items {
            TableCell::render_to(&container, &surface, mode, subject.as_ref(), status);
        }

        let controls_visible = mode == AppMode::Edit;
        for button in row_buttons {
            button.set_visible(controls_visible);
        }
        for button in column_buttons {
            button.set_visible(controls_visible);
        }
    }

    fn emit_status_changed(state: &Rc<RefCell<ViewState>>, statistics: AttendanceStatistics) {
        let callbacks = {
            let view_state = state.borrow();
            view_state.on_status_change.clone()
        };
        for callback in callbacks {
            callback(statistics);
        }
    }
}
