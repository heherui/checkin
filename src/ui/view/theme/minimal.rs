use iced::{
    theme::{self, Base},
    widget::{button, text_input},
    Border, Color,
};

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MinimalTheme;

// =========================================================================
// Base Theme trait for MinimalTheme
// =========================================================================
impl Base for MinimalTheme
{
    fn default(_preference: theme::Mode) -> Self
    {
        Self
    }

    fn mode(&self) -> theme::Mode
    {
        theme::Mode::Light
    }

    fn base(&self) -> theme::Style
    {
        theme::Style {
            background_color: Color::WHITE,
            text_color: Color::BLACK,
        }
    }

    fn palette(&self) -> Option<theme::Palette>
    {
        Some(theme::Palette {
            background: Color::WHITE,
            text: Color::BLACK,
            primary: Color::BLACK,
            success: Color::from_rgb(0.0, 0.8, 0.0),
            danger: Color::from_rgb(0.8, 0.0, 0.0),
            warning: Color::from_rgb(0.9, 0.5, 0.0),
        })
    }

    fn name(&self) -> &str
    {
        "MinimalTheme"
    }
}

// =========================================================================
// Customized Themes for Widgets via Catalog
// ======================================================================
impl button::Catalog for MinimalTheme
{
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: button::Status) -> button::Style
    {
        let base_style = button::Style {
            background: Some(iced::Background::Color(Color::WHITE)), // 白色背景
            text_color: Color::BLACK,                                // 黑色文字
            border: Border {
                color: Color::BLACK, // 黑色边框/描边
                width: 1.0,          // 1像素细线
                radius: 0.0.into(),  // 纯直角
            },
            shadow: iced::Shadow::default(),
            snap: false,
        };

        // 处理悬浮、点击时的响应
        match status {
            button::Status::Hovered => button::Style {
                text_color: Color::from_rgb(0.4, 0.4, 0.4), // 悬浮时变成深灰
                border: Border {
                    color: Color::from_rgb(0.4, 0.4, 0.4),
                    ..base_style.border
                },
                ..base_style
            },
            button::Status::Pressed => button::Style {
                text_color: Color::from_rgb(0.6, 0.6, 0.6), // 点击时变成浅灰
                border: Border {
                    color: Color::from_rgb(0.6, 0.6, 0.6),
                    ..base_style.border
                },
                ..base_style
            },
            _ => base_style,
        }
    }
}

impl text_input::Catalog for MinimalTheme
{
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, _status: text_input::Status) -> text_input::Style
    {
        text_input::Style {
            background: iced::Background::Color(Color::WHITE),
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: 0.0.into(), // 直角
            },
            icon: Color::BLACK,
            placeholder: Color::from_rgb(0.6, 0.6, 0.6),
            value: Color::BLACK,
            selection: Color::from_rgb(0.8, 0.8, 0.8),
        }
    }
}
