use chrono::NaiveDateTime;

use crate::{storage::TableData, ui::app::app_state::{AppDialog, ImportFormat}};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Message 
{
    ShowDialog(AppDialog),
    CloseDialog(AppDialog),

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
        id: u128,
        time: NaiveDateTime,
    },

    CopyStatistics,

    SelectImportFormat(ImportFormat),
}