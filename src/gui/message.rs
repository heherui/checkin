use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum Message 
{
    ShowCheckinDialog,
    CloseCheckinDialog,

    ConfirmCheckin { 
        id: i32,
        time: NaiveDateTime,
    }
}