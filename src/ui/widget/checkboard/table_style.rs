use iced::{
    Font, Pixels, advanced::text,
};

#[derive(Debug)]
pub struct TableStyle
{
    pub font: Font,
    pub text_size: Pixels,
    pub shapping: text::Shaping,
}