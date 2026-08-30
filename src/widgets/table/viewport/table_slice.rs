use super::{
    Point,
    Rectangle,
};

/// A render-ready representation of a table slice.
///
/// A `TableSlice` contains precomputed rendering data for a region of a
/// table. It intentionally does not preserve the logical row and cell
/// hierarchy of [`TableViewModel`].
///
/// The data stored in a slice is intended to be consumed directly by the
/// renderer. Logical concepts such as normal cells and merged cells should
/// already have been resolved before they reach this type.
///
/// All positions are expressed in slice coordinates.
///
/// `origin` identifies the position of the viewport inside the slice. It is
/// not the position of the slice inside the complete table.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TableSlice
{
    /// The position of the viewport's top-left corner inside this slice.
    ///
    /// The renderer uses this value to translate slice coordinates into
    /// viewport coordinates:
    ///
    /// `viewport_position = slice_position - origin`
    pub origin: Point,

    /// The precomputed background regions of the slice.
    ///
    /// Background regions should already represent the final visible
    /// background after cell merging and other background composition have
    /// been resolved.
    pub backgrounds: Vec<TableSliceBackground>,

    /// The positions of table grid lines contained in the slice.
    ///
    /// Lines are stored independently from cells so that rendering does not
    /// need to reconstruct table geometry.
    pub lines: Vec<TableSliceLine>,

    /// The text elements contained in the slice.
    ///
    /// Text positions and their associated cell regions are already resolved
    /// for rendering.
    pub texts: Vec<TableSliceCellText>,
}

/// A precomputed background region in a [`TableSlice`].
///
/// Background regions represent the final regions that need to be painted by
/// the renderer. Multiple logical cells may therefore be represented by a
/// single background region.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TableSliceBackground
{
    /// The region occupied by this background.
    pub bounds: Rectangle,

    /// The background to paint in this region.
    pub background: TableSliceBackgroundKind,
}

/// The kind of background used by a [`TableSliceBackground`].
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TableSliceBackgroundKind
{
    /// No background is painted.
    None,

    /// A solid background with an RGBA color.
    Solid(TableSliceColor),
}

/// An RGBA color used by render-ready slice data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct TableSliceColor
{
    /// The red component.
    pub r: u8,

    /// The green component.
    pub g: u8,

    /// The blue component.
    pub b: u8,

    /// The alpha component.
    pub a: u8,
}

/// A precomputed table line in a [`TableSlice`].
///
/// Lines are represented independently from cells and rows. The renderer
/// therefore does not need to calculate cell boundaries while rendering.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct TableSliceLine
{
    /// The start point of the line in slice coordinates.
    pub start: Point,

    /// The end point of the line in slice coordinates.
    pub end: Point,

    /// The width of the line in pixels.
    pub width: f32,
}

/// A precomputed text element in a [`TableSlice`].
///
/// The renderer receives the final text position directly and does not need
/// to resolve the corresponding logical cell or calculate its geometry.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TableSliceCellText
{
    /// The text to render.
    pub text: String,

    /// The position of the text in slice coordinates.
    pub position: Point,
}
