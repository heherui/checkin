#[derive(Debug, Clone)]
pub struct TableCellData
{
    pub label: Option<String>,
    pub colors: TableCellColors,
}

#[derive(Debug, Clone)]
pub struct TableCellColors
{
    pub background_color: String,
    pub label_color: String,
    pub border_color: String,
}

impl TableCellColors
{
    pub fn new(background_color: String, label_color: String, border_color: String) -> Self
    {
        return Self {
            background_color,
            label_color,
            border_color,
        };
    }
}

impl TableCellData
{
    pub fn new(label: Option<String>, colors: TableCellColors) -> Self
    {
        return Self { label, colors };
    }
}
