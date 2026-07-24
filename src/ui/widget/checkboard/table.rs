#[derive(Debug)]
pub struct Table<'a>
{
    view_model: Option<&'a TableViewModel>,
    style: TableStyle,
}

impl<'a> Table<'a>
{
    pub fn new(view_model:Option<&'a TableViewModel>, style:TableStyle )-> Self
    {
        Self { view_model, style }
    }
}

// MARK: iced widget impl
use iced::{
    Background, Border, Color, Element, Pixels, Point, Rectangle, Size, advanced::{layout::Node, renderer::Quad, text}, alignment, border::Radius,
};

use crate::ui::widget::checkboard::{table_style::TableStyle, view_model::TableViewModel};

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
                    color: Color::from_rgb(1.0, 0.0, 0.0),
                    width: 2.0,
                    radius: Radius::default(),
                },
                ..Quad::default()
            },
            Color::from_rgb(0.0, 1.0, 0.0),
        );
        renderer.fill_text(
            text::Text {
                content: "import table data to show the table".into(),
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
        let bounds = layout.bounds();

        let row_count = view_model.row_count;
        let column_count = view_model.column_count;

        let cell_size = Size::new(
            bounds.width / column_count as f32,
            bounds.height / row_count as f32,
        );

        for (y, row) in view_model.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_bounds = Rectangle {
                    x: bounds.x + cell_size.width * x as f32,
                    y: bounds.y + cell_size.height * y as f32,
                    width: cell_size.width,
                    height: cell_size.height,
                };

                //let scale = |v: f32| 0.25 + v * 0.5;
                renderer.fill_quad(
                    Quad {
                        bounds: cell_bounds,
                        border: Border {
                            width: 2.0,
                            radius: 0.0.into(),
                            color: Color::from_rgb8(231, 103, 103),
                        },
                        shadow: Default::default(),
                        snap: false,
                    },
                    // Background::Color(Color::from_rgb(
                    //     scale(x as f32 / column_count as f32),
                    //     scale(y as f32 / row_count as f32),
                    //     scale((x + y) as f32 / (row_count + column_count) as f32),
                    // )),
                    Background::Color(Color::from_rgb8(222, 145, 145)),
                );

                renderer.fill_text(
                    text::Text { 
                        content: cell.label.clone(),
                        bounds: Size::new(cell_size.width, cell_size.height),
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