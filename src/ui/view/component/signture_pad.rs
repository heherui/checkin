use iced::{
    advanced::{
        layout::Node,
        renderer::{self, Quad},
    },
    border::Radius,
    widget::{container, stack, text},
    Border, Color, Element,
    Length::Fill,
    Size,
};

use crate::ui::message::Message;

pub fn signture_pad<'a>() -> Element<'a, Message>
{
    let label = container(text("signture_pad")).center(Fill);
    let signture_pad = SignturePad {};

    container(stack![signture_pad, label])
        .center(Fill)
        .into()
}

pub struct SignturePad {}

impl<Message, Theme, Render> iced::advanced::Widget<Message, Theme, Render> for SignturePad
where
    Render: iced::advanced::Renderer,
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
        let size = limits.resolve(Fill, Fill, Size::new(400.0, 250.0));
        Node::new(size)
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
            Color::TRANSPARENT,
        );
    }
}

impl<Message, Theme, Renderer> From<SignturePad> for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(signture_pad: SignturePad) -> Self
    {
        Self::new(signture_pad)
    }
}
