pub struct Table<'a, Message: 'a>
{
    view_model: Option<&'a TableViewModel>,
    style: TableStyle,
    on_press: Option<Box<dyn Fn(u128) -> Message + 'a>>,
}

impl<'a, Message> Table<'a, Message>
{
    pub fn new(view_model: Option<&'a TableViewModel>, style: TableStyle) -> Self
    {
        Self {
            view_model,
            style,
            on_press: None,
        }
    }
}

// Table Builder APIs
impl<'a, Message> Table<'a, Message>
{
    pub fn on_press(mut self, f: impl Fn(u128) -> Message + 'a) -> Self
    {
        self.on_press = Some(Box::new(f));
        self
    }
}

use std::{eprintln, println};

// iced wiget impl
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

use crate::ui::widget::table::{
    table_style::TableStyle,
    view_model::{CellSize, TableCellViewModel, TableViewModel},
};

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
        if let Some(view_model) = self.view_model {
            self.draw_table_view(renderer, layout, view_model);
        } else {
            self.draw_empty_view(renderer, layout);
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
        if let Mouse(ButtonPressed(button)) = event {
            if *button != Button::Left {
                return;
            };
            if !cursor.is_over(layout.bounds()) {
                return;
            };
            if let Some(p) = cursor.position() {
                println!("mouse down: ({},{})", p.x, p.y);
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
                println!("mouse down: ({},{})", p.x, p.y);
            }
        };
    }
}

/// private drawing helpers
impl<'a, Message> Table<'a, Message>
{
    fn draw_empty_view<Render>(&self, renderer: &mut Render, layout: iced::advanced::Layout<'_>)
    where
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
                content: "None Table Data".into(),
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

    fn draw_table_view<Render>(
        &self,
        renderer: &mut Render,
        layout: iced::advanced::Layout<'_>,
        view_model: &'a TableViewModel,
    ) where
        Render: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
    {
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
                self.draw_cell(renderer, cell_bounds, cell_view_model);

                // gen start point.x for next cell
                cell_start_point.x += cell_width;
            }

            // gen start point.y for next row of cells
            cell_start_point.x = first_cell_start_point.x;
            cell_start_point.y += cell_height;
        }
    }

    fn draw_cell<Render>(
        &self,
        renderer: &mut Render,
        cell_bounds: Rectangle,
        cell_view_model: &TableCellViewModel,
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
                size: self.style.text_size,
                line_height: Default::default(),
                font: self.style.font,
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                shaping: self.style.shapping,
                wrapping: text::Wrapping::None,
            },
            Point::new(cell_bounds.center_x(), cell_bounds.center_y()),
            Color::BLACK,
            cell_bounds,
        );
    }
}

/// cell meature helpers
impl<'a, Message> Table<'a, Message>
{
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
            for width_offset in &view_model.cell_width_offsets {
                width_offsets += width_offset;
            }
            for height_offset in &view_model.cell_height_offsets {
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
        return Size { width: 0.0, height: 0.0 };
    }
}

/// private mouse event helpers
impl<'a, Message> Table<'a, Message> {}

/// MARK: Table into Element
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
