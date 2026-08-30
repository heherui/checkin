use std::matches;

/// The visual style used when interpreting a table view model.
///
/// `TableStyle` belongs to the parsing layer rather than the view model
/// because layout information in the view model must not depend on how the
/// table is rendered.
#[derive(Debug, Clone, Copy)]
pub struct TableStyle
{
    /// The grid-line style used between adjacent cells.
    pub grid_line: TableGridLine,

    /// Whether an outer border should be rendered.
    ///
    /// Outer borders are currently not handled by the table parser.
    pub draw_outer_border: bool,
}

/// Describes how a table grid line participates in layout.
#[derive(Debug, Clone, Copy)]
pub enum TableGridLine
{
    /// The grid line does not occupy layout space.
    ///
    /// Cell boundaries remain coincident, and the grid line is rendered over
    /// the logical boundary.
    Visual(f32),

    /// The grid line occupies layout space.
    ///
    /// Adjacent cell boundaries are separated by the grid line.
    Sized(f32),
}

impl TableGridLine
{
    /// Returns the width of the grid line.
    pub fn width(self) -> f32
    {
        match self
        {
            Self::Visual(width) | Self::Sized(width) => width,
        }
    }

    /// Returns whether the grid line occupies layout space.
    pub fn is_sized(self) -> bool
    {
        matches!(self, Self::Sized(_))
    }
}