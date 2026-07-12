use iced::{Border, Color, Theme, widget::{Button, button, text}};

pub fn plain_button<'a, Message>(
    label: impl Into<String>,
) -> Button<'a, Message, Theme>
where
    Message: Clone,
{
    button(text(label.into()))
        .padding([8, 12])
        .style(|theme:&'_ Theme, status| {
            use iced::widget::button;
            match status {
                button::Status::Hovered => {
                    button::Style {
                        background: None,
                        border: Border {
                            width: 2.0,
                            radius: 6.0.into(),
                            color: theme.palette().text,
                        },
                        ..Default::default()
                    }
                }
                _ => {
                    button::Style {
                        background: None,
                        border: Border {
                            width: 2.0,
                            radius: 6.0.into(),
                            color: Color::TRANSPARENT,
                        },
                        ..Default::default()
                    }
                }
            }
        })
}