use chrono::NaiveDateTime;

use crate::storage::TableData;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Message 
{
    ShowCheckinDialog,
    CloseCheckinDialog,

    LoadTableData,
    LoadTableDataResult {
        result: Result<TableData, String>,
    },

    SaveTableData {
        data: TableData,
    },
    SaveTableDataResult {
        result: Result<(), String>,
    },

    ImportTableDataFromExcel,

    ConfirmCheckin {
        id: i32,
        time: NaiveDateTime,
    },
}