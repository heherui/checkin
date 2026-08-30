/// A point in table-slice coordinates.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct Point
{
    /// The horizontal coordinate.
    pub x: f32,

    /// The vertical coordinate.
    pub y: f32,
}

impl Point
{
    /// Creates a point.
    pub const fn new(x: f32, y: f32) -> Self
    {
        Self { x, y }
    }
}

/// An axis-aligned rectangle in table-slice coordinates.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct Rectangle
{
    /// The horizontal coordinate of the left edge.
    pub x: f32,

    /// The vertical coordinate of the top edge.
    pub y: f32,

    /// The width of the rectangle.
    pub width: f32,

    /// The height of the rectangle.
    pub height: f32,
}

impl Rectangle
{
    /// Creates a rectangle.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self
    {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}