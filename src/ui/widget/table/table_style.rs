use iced::{advanced::text, widget::text::Shaping::Advanced, Font, Pixels};

#[derive(Debug)]
pub struct TableStyle
{
    pub label_font: Font,
    pub label_size: Pixels,
    pub shapping: text::Shaping,
}

impl TableStyle
{
    pub fn default() -> Self
    {
        Self {
            label_font: Font::default(),
            label_size: Pixels(16.0),
            shapping: Advanced,
        }
    }
}
