use iced::{Background, Color, Element, Length::Fill, Theme, widget::container};

use crate::ui::app::Message;

pub fn dialog_overlay<'a>(content:Element<'a,Message>)-> Element<'a, Message>
{
    let dialog_overlay = container({
        container(content)
            .padding(20)
            .max_height(350)
            .max_width(500)
            .style(|theme:&Theme| container::Style {
                // background: Some(Background::Color(Color::WHITE)),
                background: Some(Background::Color(theme.palette().background)),
                border: iced::Border {
                    width: 1.0,
                    radius: 8.0.into(),
                    // color: Color::from_rgb8(200, 200, 200),
                    color: theme.palette().primary,
                },

                ..Default::default()
            })
    })
    .width(Fill)
    .height(Fill)
    .center(Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        })),
        ..Default::default()
    });

    return dialog_overlay.into();
}
