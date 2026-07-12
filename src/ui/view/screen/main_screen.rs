use iced::{
    widget::{button, column, container, row, stack},
    Background, Color, Element,
    Length::Fill,
};

use crate::ui::{
    app_state::AppState,
    message::Message,
    view::component::{
        checkboard::checkboard, checkin_dialog::checkin_dialog, statistics::statistics,
    },
};

pub fn main_screen(app_state: &AppState) -> Element<'_, Message>
{
    let statistics = statistics();
    let checkboard = checkboard(app_state.table_view_model.as_ref());
    let developer_inspector = row![
        button("ShowCheckinDialog").on_press(Message::ShowCheckinDialog),
        button("SaveToConfig"),
        button("LoadFromConfig"),
        button("importXlsx").on_press(Message::ImportTableDataFromExcel),
        button("CopyStatistics"),
    ].spacing(3);

    let root = container(column![
        container(row![
            container(statistics),
            container(developer_inspector),
        ])
        .max_height(200),
        container(checkboard),
    ].spacing(3))
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
