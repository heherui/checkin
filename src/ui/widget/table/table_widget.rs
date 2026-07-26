use crate::ui::widget::table::{
    table_style::TableStyle,
    table_view_model::{CellSize, TableCellViewModel, TableViewModel},
};
use iced::{
    advanced::{layout::Node, mouse, renderer::Quad, text, widget::Tree},
    alignment,
    border::Radius,
    mouse::{
        Button,
        Event::{ButtonPressed, ButtonReleased},
    },
    Background, Border, Color, Element,
    Event::Mouse,
    Pixels, Point, Rectangle, Size,
};
use std::{eprintln, println};

/// Table Widget
pub struct Table<'a, Message: 'a>
{
    view_model: Option<&'a TableViewModel>,
    table_style: TableStyle,
    on_press: Option<Box<dyn Fn(u128) -> Message + 'a>>,
}

/// Table Builder APIs
impl<'a, Message> Table<'a, Message>
{
    pub fn new() -> Self
    {
        Self {
            view_model: None,
            table_style: TableStyle::default(),
            on_press: None,
        }
    }

    pub fn view_model(mut self, view_model: &'a TableViewModel) -> Self
    {
        self.view_model = Some(view_model);
        return self;
    }

    pub fn style(mut self, table_style: TableStyle) -> Self
    {
        self.table_style = table_style;
        return self;
    }

    pub fn on_press(mut self, f: impl Fn(u128) -> Message + 'a) -> Self
    {
        self.on_press = Some(Box::new(f));
        self
    }
}

// Table iced wiget impl
impl<'a, Message, Theme, Render> iced::advanced::Widget<Message, Theme, Render>
    for Table<'a, Message>
where
    Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> iced::Size<iced::Length>
    {
        iced::Size::new(iced::Fill, iced::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Render,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node
    {
        let max = limits.max();

        Node::new(Size::new(max.width.floor(), max.height.floor()))
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut Render,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    )
    {
        if let (Some(table_layout), Some(table_cellsref)) =
            (&self.table_layout, &self.table_cellsref)
        {
            Self::draw_table_view(renderer, table_layout, table_cellsref, &self.table_style);
        } else {
            self.draw_error_view(renderer, layout, "Error: view model is None for table");
        }
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Render,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    )
    {
        // keep layout and cellsref up to date


        // handle mouse events
        if let Mouse(ButtonPressed(button)) = event {
            if *button != Button::Left {
                return;
            };
            if !cursor.is_over(layout.bounds()) {
                return;
            };
            if let (Some(p), Some(table_layout)) = (cursor.position(), &self.table_layout) {
                println!("mouse down: ({},{})", p.x, p.y);
                let hit = Self::test_hit_cell(table_layout, p, layout);
                if let Some((x, y)) = hit {
                    println!("hit: ({x}, {y})");
                };
            }
        };

        if let Mouse(ButtonReleased(button)) = event {
            if *button != Button::Left {
                return;
            };
            if !cursor.is_over(layout.bounds()) {
                return;
            };
            if let Some(p) = cursor.position() {
                println!("mouse up: ({},{})", p.x, p.y);
            }
        };
    }
}

/// private drawing helpers
impl<'a, Message> Table<'a, Message>
{
    fn draw_error_view<Render>(
        &self,
        renderer: &mut Render,
        layout: iced::advanced::Layout<'_>,
        msg: &str,
    ) where
        Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
    {
        let bounds = layout.bounds();

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    color: Color::from_rgb8(246, 245, 244),
                    width: 2.0,
                    radius: Radius::default(),
                },
                ..Quad::default()
            },
            Color::from_rgb8(246, 248, 252),
        );
        renderer.fill_text(
            text::Text {
                content: msg.into(),
                bounds: bounds.size(),
                size: Pixels(24.0),
                line_height: Default::default(),
                font: Render::Font::default(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::None,
            },
            Point::new(
                bounds.x + bounds.width / 2.0,
                bounds.y + bounds.height / 2.0,
            ),
            Color::BLACK,
            bounds,
        );
        return;
    }

    #[deprecated(since = "0.1.0", note = "replace with draw_table_view")]
    #[allow(unused)]
    fn draw_table_view_with_view_model_directly<Render>(
        &self,
        renderer: &mut Render,
        layout: iced::advanced::Layout<'_>,
        view_model: &'a TableViewModel,
    ) where
        Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
    {
        #[allow(deprecated)]
        let cell_default_size = self.default_cell_size(layout);

        let width_offsets = &view_model.cell_width_offsets;
        let height_offsets = &view_model.cell_height_offsets;

        let first_cell_start_point = Point {
            x: layout.bounds().x + view_model.start_point.x,
            y: layout.bounds().y + view_model.start_point.y,
        };
        let mut cell_start_point = first_cell_start_point;

        for (y, row) in view_model.rows.iter().enumerate() {
            let mut cell_height = cell_default_size.height;
            if y < height_offsets.len() {
                cell_height += height_offsets[y];
            };

            for (x, cell_view_model) in row.iter().enumerate() {
                let mut cell_width = cell_default_size.width;
                if x < width_offsets.len() {
                    cell_width += width_offsets[x];
                };
                let cell_bounds = Rectangle {
                    x: cell_start_point.x,
                    y: cell_start_point.y,
                    width: cell_width,
                    height: cell_height,
                };
                Self::draw_cell(renderer, cell_bounds, cell_view_model, &self.table_style);

                // gen start point.x for next cell
                cell_start_point.x += cell_width;
            }

            // gen start point.y for next row of cells
            cell_start_point.x = first_cell_start_point.x;
            cell_start_point.y += cell_height;
        }
    }

    fn draw_table_view<Render>(
        renderer: &mut Render,
        table_layout: &TableLayout,
        table_cellsref: &TableCellsRef,
        table_style: &TableStyle,
    ) where
        Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
    {
        let mut cell_start_y = table_layout.start_point.y;
        let mut cell_start_x = table_layout.start_point.x;
        for (y, cell_height) in table_layout.cell_heights.iter().enumerate() {
            for (x, cell_width) in table_layout.cell_widths.iter().enumerate() {
                let cell_view_model = table_cellsref.access(x, y);
                let cell_bounds = Rectangle::new(
                    Point::new(cell_start_x, cell_start_y),
                    Size::new(*cell_width, *cell_height),
                );
                Self::draw_cell(renderer, cell_bounds, cell_view_model, table_style);
            }
            cell_start_y += cell_height;
            cell_start_x = table_layout.start_point.x;
        }
    }

    fn draw_cell<Render>(
        renderer: &mut Render,
        cell_bounds: Rectangle,
        cell_view_model: &TableCellViewModel,
        table_style: &TableStyle,
    ) where
        Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
    {
        let cell_bounds = Rectangle {
            x: cell_bounds.x.round(),
            y: cell_bounds.y.round(),
            width: cell_bounds.width.round(),
            height: cell_bounds.height.round(),
        };

        let border_color = match cell_view_model.border_color {
            Some(color) => color,
            None => cell_view_model.background_color,
        };

        renderer.fill_quad(
            Quad {
                bounds: cell_bounds,
                border: Border {
                    width: 2.0,
                    radius: 0.0.into(),
                    color: border_color,
                },
                shadow: Default::default(),
                snap: false,
            },
            Background::Color(cell_view_model.background_color),
        );

        renderer.fill_text(
            text::Text {
                content: cell_view_model.label.clone(),
                bounds: Size::new(cell_bounds.width, cell_bounds.height),
                size: table_style.label_size,
                line_height: Default::default(),
                font: table_style.label_font,
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                shaping: table_style.shapping,
                wrapping: text::Wrapping::None,
            },
            Point::new(cell_bounds.center_x(), cell_bounds.center_y()),
            Color::BLACK,
            cell_bounds,
        );
    }
}

/// private cell meature helpers
impl<'a, Message> Table<'a, Message>
{
    #[deprecated(
        since = "0.1.0",
        note = "replace with table layout drawing instead of view model drawing"
    )]
    #[allow(unused)]
    fn default_cell_size(&self, layout: iced::advanced::Layout<'_>) -> Size
    {
        if self.view_model.is_none() {
            eprintln!("fail tu meature cell width: view model is none");
            return Size {
                width: 0.0,
                height: 0.0,
            };
        };
        let view_model = self.view_model.unwrap();
        let cell_default_size = view_model.cell_default_size;

        if let CellSize::Fixed(size) = cell_default_size {
            return size;
        };

        let rows = &view_model.rows;
        if rows.is_empty() {
            eprintln!("fail tu meature cell width: no data for rows to meature");
            return Size {
                width: 0.0,
                height: 0.0,
            };
        };
        let row_count = rows.len();
        let column_count = rows[0].len();

        if let CellSize::StretchToFit = cell_default_size {
            let mut width_offsets: f32 = 0.0;
            let mut height_offsets: f32 = 0.0;
            for width_offset in view_model.cell_width_offsets.iter() {
                width_offsets += width_offset;
            }
            for height_offset in view_model.cell_height_offsets.iter() {
                height_offsets += height_offset;
            }
            let min_frame_width = layout.bounds().width - width_offsets;
            let min_frame_height = layout.bounds().height - height_offsets;
            return Size {
                width: min_frame_width / column_count as f32,
                height: min_frame_height / row_count as f32,
            };
        };

        eprintln!("unsupported cell size, fail to meature default cell width.");
        return Size {
            width: 0.0,
            height: 0.0,
        };
    }
}

/// private mouse event helpers
impl<'a, Message> Table<'a, Message>
{
    fn test_hit_cell(
        table_layout: &TableLayout,
        position: Point,
        frame: iced::advanced::Layout<'_>,
    ) -> Option<(usize, usize)>
    {
        let frame = frame.bounds();
        if !frame.contains(position) {
            return None;
        };

        // compute index (x,y) for the cell cursor hited.
        // algorithm: for-each to add cell widths up, while the cursor is smaller than the
        // added width, that is the cell the cursor is. similarly foreach to add heights up
        // to find which y cell index the cursor is.
        let mut traversed_width: f32 = 0.0;
        let mut traversed_height: f32 = 0.0;
        let mut result: Option<(usize, usize)> = None;
        for (x, width) in table_layout.cell_widths.iter().enumerate() {
            if !(position.x < traversed_width) {
                continue;
            };
            for (y, height) in table_layout.cell_heights.iter().enumerate() {
                if position.y < traversed_height {
                    result = Some((x, y));
                    break;
                }
                traversed_height += height;
            }
            traversed_width += width;
        }
        return result;
    }
}

/// Table into Element
impl<'a, Message, Theme, Renderer> From<Table<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
{
    fn from(table: Table<'a, Message>) -> Self
    {
        Self::new(table)
    }
}
