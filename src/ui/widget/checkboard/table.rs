#[derive(Debug)]
pub struct Table<'a>
{
    view_model: Option<&'a TableViewModel>,
    style: TableStyle,
}

impl<'a> Table<'a>
{
    pub fn new(view_model: Option<&'a TableViewModel>, style: TableStyle) -> Self
    {
        Self { view_model, style }
    }
}

// MARK: iced widget impl
use iced::{
    advanced::{layout::Node, renderer::Quad, text},
    alignment,
    border::Radius,
    Background, Border, Color, Element, Pixels, Point, Rectangle, Size,
};

use crate::ui::widget::checkboard::{
    table_style::TableStyle,
    view_model::{TableCellViewModel, TableViewModel},
};

impl<'a, Message, Theme, Render> iced::advanced::Widget<Message, Theme, Render> for Table<'a>
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
        Node::new(limits.max())
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
}

// private drawing helpers
impl<'a> Table<'a>
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
        let width_offsets = &view_model.cell_width_offsets;
        let height_offsets = &view_model.cell_height_offsets;

        let first_cell_start_point = Point {
            x: layout.bounds().x + view_model.start_point.x,
            y: layout.bounds().y + view_model.start_point.y,
        };
        let mut cell_start_point = first_cell_start_point;

        for (y, row) in view_model.rows.iter().enumerate() {

            let mut cell_height = view_model.cell_default_height;
            if y < height_offsets.len() {
                cell_height += height_offsets[y];
            };

            for (x, cell_view_model) in row.iter().enumerate() {

                let mut cell_width = view_model.cell_default_width;
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
                    color: border_color
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

impl<'a, Message, Theme, Renderer> From<Table<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + text::Renderer + text::Renderer<Font = iced::Font>,
{
    fn from(table: Table<'a>) -> Self
    {
        Self::new(table)
    }
}
