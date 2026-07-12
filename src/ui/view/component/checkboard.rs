use iced::{
    advanced::{
        layout::Node,
        renderer::{self, Quad},
        text,
    },
    alignment,
    border::Radius,
    Background, Border, Color, Element, Pixels, Point, Rectangle, Size,
};

use crate::{
    storage::{PersonId, TableData},
    ui::message::Message,
};

pub fn checkboard<'a>(table_view_model: Option<&'a TableViewModel>) -> Element<'a, Message>
{
    Table {
        view_model: table_view_model,
    }
    .into()
}

#[derive(Debug)]
pub struct Table<'a>
{
    view_model: Option<&'a TableViewModel>,
}

#[derive(Debug)]
pub struct TableViewModel
{
    row_count: u32,
    column_count: u32,
    rows: Vec<Vec<TableCellViewModel>>,
}

#[derive(Debug)]
pub struct TableCellViewModel
{
    label: String,
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
        Render: iced::advanced::Renderer + text::Renderer,
        Render::Font: Default,
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
        Render: iced::advanced::Renderer + text::Renderer,
        Render::Font: Default,
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

                let scale = |v: f32| 0.25 + v * 0.5;
                renderer.fill_quad(
                    Quad {
                        bounds: cell_bounds,
                        border: Default::default(),
                        shadow: Default::default(),
                        snap: false,
                    },
                    Background::Color(Color::from_rgb(
                        scale(x as f32 / column_count as f32),
                        scale(y as f32 / row_count as f32),
                        scale((x + y) as f32 / (row_count + column_count) as f32),
                    )),
                );

                renderer.fill_text(
                    text::Text {
                        content: cell.label.clone(),
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

use std::collections::HashMap;

impl TableViewModel
{
    pub fn load_from(table_data: TableData) -> Self
    {
        let row_count = table_data.table_layout.row_count;
        let column_count = table_data.table_layout.column_count;

        let mut rows = Vec::with_capacity(row_count as usize);
        for _ in 0..row_count {
            let mut row = Vec::with_capacity(column_count as usize);
            for _ in 0..column_count {
                row.push(TableCellViewModel {
                    label: String::new(),
                });
            }
            rows.push(row);
        }

        let person_map: HashMap<PersonId, &str> = table_data
            .personnel
            .iter()
            .map(|p| (p.id, p.name.as_str()))
            .collect();

        // Fill cells that have a seat assignment
        for assignment in table_data.table_layout.seats_assignment {
            let x = assignment.coordinate.x.saturating_sub(1) as usize;
            let y = assignment.coordinate.y.saturating_sub(1) as usize;

            if y < rows.len() && x < rows[y].len() {
                if let Some(name) = person_map.get(&assignment.person_id) {
                    rows[y][x].label = name.to_string();
                }
            }
        }

        TableViewModel {
            row_count,
            column_count,
            rows,
        }
    }
}
