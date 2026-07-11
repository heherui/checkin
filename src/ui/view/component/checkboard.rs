use iced::{
    advanced::{
        layout::Node,
        renderer::{self, Quad},
        text,
    },
    alignment,
    Background, Color, Element,
    Pixels, Point, Rectangle, Size,
};

use crate::ui::message::Message;

pub fn checkboard<'a>() -> Element<'a, Message>
{
    Table {
        data: &TableData {
            row_count: 8,
            column_count: 10,
        },
    }
    .into()
}

#[derive(Debug)]
pub struct Table<'a>
{
    data: &'a TableData,
}

#[derive(Debug)]
pub struct TableData
{
    row_count: u32,
    column_count: u32,
}

impl<'a, Message, Theme, Render> iced::advanced::Widget<Message, Theme, Render> for Table<'a>
where
    Render: iced::advanced::Renderer + text::Renderer,
    Render::Font: Default,
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
        let bounds = layout.bounds();

        let row_count = self.data.row_count;
        let column_count = self.data.column_count;

        let cell_size = Size::new(
            bounds.width / column_count as f32,
            bounds.height / row_count as f32,
        );

        for x in 0..column_count {
            for y in 0..row_count {
                let cell_bounds = Rectangle {
                    x: bounds.x + cell_size.width * x as f32,
                    y: bounds.y + cell_size.height * y as f32,
                    width: cell_size.width,
                    height: cell_size.height,
                };

                renderer.fill_quad(
                    Quad {
                        bounds: cell_bounds,
                        border: Default::default(),
                        shadow: Default::default(),
                        snap: false,
                    },
                    Background::Color(Color::from_rgb(
                        x as f32 / column_count as f32,
                        y as f32 / row_count as f32,
                        ((x + y) as f32) / (row_count + column_count) as f32,
                    )),
                );

                renderer.fill_text(
                    text::Text {
                        content: format!("{x},{y}"),
                        bounds: Size::new(cell_size.width, cell_size.height),
                        size: Pixels(14.0),
                        line_height: Default::default(),
                        font: Render::Font::default(),
                        align_x: alignment::Horizontal::Center.into(),
                        align_y: alignment::Vertical::Center,
                        shaping: text::Shaping::Basic,
                        wrapping: text::Wrapping::None,
                    },
                    Point::new(cell_bounds.center_x(), cell_bounds.center_y()),
                    Color::BLACK,
                    cell_bounds,
                );
            }
        }

        // renderer.fill_quad(
        //     Quad {
        //         bounds,
        //         border: Border {
        //             color: Color::from_rgb(1.0, 0.0, 0.0),
        //             width: 2.0,
        //             radius: Radius::default(),
        //         },
        //         ..Quad::default()
        //     },
        //     Color::from_rgb(0.0, 1.0, 0.0),
        // );
    }
}

impl<'a, Message, Theme, Renderer> From<Table<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer,
    Renderer::Font: Default,
{
    fn from(table: Table<'a>) -> Self
    {
        Self::new(table)
    }
}
