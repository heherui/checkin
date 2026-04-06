use gtk4::{
    glib::{self, subclass::types::{ObjectSubclass, ObjectSubclassExt}},
    traits::GridExt,
};
use uuid::Uuid;

use crate::{core::Position, ui::component::table::data::{TableElement, TableElementMap}};
use crate::ui::component::{
    TableCell,
    TableCellColors,
    TableCellData,
    TablePersonStatus,
    TablePersonsStatusMap,
};

mod imp
{
    use std::{cell::RefCell, collections::HashMap};

    use gtk4::{
        glib::{
            self,
            subclass::{
                object::{ObjectImpl, ObjectImplExt},
                types::{ObjectSubclass, ObjectSubclassExt},
            },
        },
        subclass::{box_::BoxImpl, widget::WidgetImpl},
        traits::{BoxExt, GridExt, WidgetExt},
    };
    use uuid::Uuid;

    use crate::{core::Position, ui::component::TableCell};

    #[derive(Default)]
    pub struct Table
    {
        pub grid: RefCell<Option<gtk4::Grid>>,
        pub cells: RefCell<HashMap<Position,TableCell>>,
        pub person_cells: RefCell<Vec<Position>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Table
    {
        type Type = super::Table;
        type ParentType = gtk4::Box;

        const NAME: &'static str = "CheckinTable";
    }

    impl ObjectImpl for Table
    {
        fn constructed(&self)
        {
            self.parent_constructed();

            let obj = self.obj();
            obj.set_hexpand(true);
            obj.set_vexpand(true);
            obj.set_halign(gtk4::Align::Fill);
            obj.set_valign(gtk4::Align::Fill);

            let grid = gtk4::Grid::new();
            grid.set_row_spacing(1);
            grid.set_column_spacing(1);
            grid.set_row_homogeneous(false);
            grid.set_column_homogeneous(false);
            grid.set_hexpand(true);
            grid.set_vexpand(true);
            grid.set_halign(gtk4::Align::Fill);
            grid.set_valign(gtk4::Align::Fill);

            obj.append(&grid);

            *self.grid.borrow_mut() = Some(grid);
        }
    }
    impl WidgetImpl for Table {}
    impl BoxImpl for Table {}
}

glib::wrapper! {
    pub struct Table(ObjectSubclass<imp::Table>)
        @extends gtk4::Widget, gtk4::Box;
}

impl Table
{
    pub fn new() -> Self
    {
        return glib::Object::new::<Self>();
    }
}

/// table element map
impl Table
{
    pub fn set_element_map(&self, element_map: TableElementMap) -> Result<(), String>
    {
        let table = imp::Table::from_obj(&self);

        for (y, row) in element_map.rows.iter().enumerate()
        {
            for (x, element) in row.iter().enumerate()
            {
                if let Err(e) = Self::set_element_at(table, x as i32, y as i32, element)
                {
                    return Err(format!(
                        "fail to set table element at {position} while setting the entire table element map:\n\t{e}"
                    ));
                };
            }
        }

        return Ok(());
    }

    fn set_element_at(
        table: &imp::Table,
        position:Position,
        element: &TableElement,
    ) -> Result<(), String>
    {
        let table_grid = table.grid.borrow();
        let table_grid = match table_grid.as_ref()
        {
            Some(grid) => grid,
            None =>
            {
                return Err(format!(
                    "cannot access to grid of table while set up a cell at {position}"
                ))
            }
        };

        if let &TableElement::Person { uuid, person } = &element
        {
            let cell_data =
                TableCellData::from_person_with_status(&person, &TablePersonStatus::default());
            let cell = match TableCell::from(cell_data)
            {
                Ok(cell) => cell,
                Err(e) => return Err(format!("fail to build cell at {position}: \n\t{e}")),
            };

            if let Err(e) = Self::set_grid_cell_at(table_grid, position, &cell)
            {
                return Err(format!(
                    "canot set the grid cell properly at {position}: \n\t{e}"
                ));
            };

            let mut cell_tracker_by_uuid = table.person_person_cells_by_id.borrow_mut();
            cell_tracker_by_uuid.insert(*uuid, cell);

            return Ok(());
        };

        if let &TableElement::Block { name } = &element
        {
            let cell_data = TableCellData::new_block(name.clone());
            let cell = match TableCell::from(cell_data)
            {
                Ok(cell) => cell,
                Err(e) => return Err(format!("fail to build cell at {position}: \n\t{e}")),
            };
            if let Err(e) = Self::set_grid_cell_at(table_grid, position, &cell)
            {
                return Err(format!(
                    "canot set the grid cell properly at {position}: \n\t{e}"
                ));
            };

            return Ok(());
        };

        if let &TableElement::Transparent = &element
        {
            let cell_data = TableCellData::new_transparent();
            let cell = match TableCell::from(cell_data)
            {
                Ok(cell) => cell,
                Err(e) => return Err(format!("fail to build cell at {position}: \n\t{e}")),
            };
            if let Err(e) = Self::set_grid_cell_at(table_grid, position, &cell)
            {
                return Err(format!(
                    "canot set the grid cell properly at {position}: \n\t{e}"
                ));
            };

            return Ok(());
        };

        return Err(format!(
            "this cell type does not support being set in a table {position}"
        ));
    }

    fn set_grid_cell_at(grid: &gtk4::Grid, x: i32, y: i32, cell: &TableCell) -> Result<(), String>
    {
        grid.attach(cell, x, y, 1, 1);
        return Ok(());
    }
}

/// table status map
impl Table
{
    pub fn set_persons_status_map(&self, map: TablePersonsStatusMap) -> Result<(), String>
    {
        let table = imp::Table::from_obj(self);
        let cell_tracker_by_id = table.person_person_cells_by_id.borrow_mut();
        for (uuid, status) in map.status
        {
            let cell = match cell_tracker_by_id.get(&uuid)
            {
                Some(cell) => cell,
                None =>
                {
                    return Err(format!("cell not found by given uuid"));
                }
            };

            if let Err(e) = Self::set_person_status_for_cell(&status, &cell)
            {
                eprintln!("fail to set person status for cell ({uuid}): \n\t{e}")
            };
        }

        return Ok(());
    }

    pub fn set_person_status_for(
        &self,
        uuid: &Uuid,
        status: &TablePersonStatus,
    ) -> Result<(), String>
    {
        let table = imp::Table::from_obj(self);
        let cell_tracker_by_id = table.person_person_cells_by_id.borrow_mut();
        let cell = match cell_tracker_by_id.get(&uuid)
        {
            Some(cell) => cell,
            None =>
            {
                return Err(format!("cell not found by given uuid"));
            }
        };
        let cell_colors = TableCellColors::new(
            status.background_color(),
            status.label_color(),
            status.border_color(),
        );
        return cell.set_colors(cell_colors);
    }

    fn set_person_status_for_cell(
        status: &TablePersonStatus,
        cell: &TableCell,
    ) -> Result<(), String>
    {
        let cell_colors = TableCellColors::new(
            status.background_color(),
            status.label_color(),
            status.border_color(),
        );
        return cell.set_colors(cell_colors);
    }
}

impl Table
{
    pub(crate) fn person_cell_by_id(&self, uuid:&Uuid)-> &TableCell
    {
        let table = imp::Table::from_obj(self);

        table.person_person_cells_by_id
    }
}