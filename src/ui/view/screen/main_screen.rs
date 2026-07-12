use iced::{
    widget::{column, container, stack},
    Background, Color, Element,
    Length::Fill,
};

use crate::ui::{
    app_state::AppState, message::Message, view::component::{
        checkboard::checkboard, checkin_dialog::checkin_dialog, developer_inspector::developer_inspector, statistics::statistics,
    },
};

pub fn main_screen(app_state: &AppState) -> Element<'_, Message>
{
    let statistics = statistics(app_state.statistics_view_model.as_ref());
    let developer_inspector = developer_inspector();
    let checkboard = checkboard(app_state.table_view_model.as_ref());

    let root = container(
        column![
            container(statistics).height(100),
            container(developer_inspector).height(50),
            container(checkboard),
        ]
        .spacing(3),
    )
    .padding(10)
    .width(Fill)
    .height(Fill);

    let overlay_dialog = container({
        container(checkin_dialog())
            .padding(20)
            .max_height(350)
            .max_width(500)
            .style(|theme| container::Style {
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

    if app_state.show_checkin_dialog {
        stack![root, overlay_dialog].into()
    } else {
        root.into()
    }
}
