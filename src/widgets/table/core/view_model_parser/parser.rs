use crate::widgets::table::{
    table_style::TableStyle,
    view_model::TableViewModel,
    viewport::{Point, Rectangle, TableSlice, TableSliceLine},
};

use super::{
    cells::{build_texts, resolve_cells, CellResolution},
    geometry::{CellRange, TableGeometry},
};

/// Converts a [`TableViewModel`] into render-ready table slice data.
///
/// The parser is the boundary where logical table information, table style,
/// and viewport state are combined.
///
/// The resulting [`TableSlice`] uses slice coordinates for all renderable
/// geometry. Its `origin` stores the viewport's top-left position inside that
/// slice.
pub struct ViewModelParser;

impl ViewModelParser
{
    /// Builds a table slice covering the cells intersecting `viewport`.
    ///
    /// The slice is aligned to logical cell boundaries. If the viewport
    /// intersects a merged cell, the slice is automatically expanded to
    /// contain the complete merged region.
    ///
    /// `viewport` is expressed in table coordinates.
    ///
    /// The returned `TableSlice::origin` is expressed in slice coordinates.
    pub fn slice(
        view_model: &TableViewModel,
        table_style: &TableStyle,
        viewport: Rectangle,
    ) -> TableSlice
    {
        if view_model.is_empty() {
            return Self::empty_slice();
        }
        let geometry = TableGeometry::new(view_model, table_style.grid_line);
        let Some(mut range) = geometry.cells_intersecting(viewport) else {
            return Self::empty_slice();
        };

        let resolution = CellResolution::new(view_model);
        resolution.expand_slice(&mut range);
        let slice_bounds = geometry.bounds(range);
        let cells = resolve_cells(view_model, &geometry, &resolution, range, slice_bounds);
        let texts = build_texts(&cells);
        let lines = build_grid_lines(view_model, &geometry, &resolution, range, slice_bounds);
        let origin = Point::new(viewport.x - slice_bounds.x, viewport.y - slice_bounds.y);

        TableSlice {
            origin,
            backgrounds: Vec::new(),
            lines,
            texts,
        }
    }

    /// Creates an empty table slice.
    fn empty_slice() -> TableSlice
    {
        TableSlice {
            origin: Point::new(0.0, 0.0),
            backgrounds: Vec::new(),
            lines: Vec::new(),
            texts: Vec::new(),
        }
    }
}

/// Builds all internal grid lines contained by a slice.
///
/// A grid line is emitted only where the two adjacent logical cells have
/// different owners. This makes internal grid lines disappear automatically
/// inside merged regions.
fn build_grid_lines(
    view_model: &TableViewModel,
    geometry: &TableGeometry,
    resolution: &CellResolution,
    range: CellRange,
    slice_bounds: Rectangle,
) -> Vec<TableSliceLine>
{
    if geometry.line_width() <= 0.0 {
        return Vec::new();
    }

    let mut lines = Vec::new();

    build_vertical_lines(
        view_model,
        geometry,
        resolution,
        range,
        slice_bounds,
        &mut lines,
    );

    build_horizontal_lines(
        view_model,
        geometry,
        resolution,
        range,
        slice_bounds,
        &mut lines,
    );

    lines
}

/// Builds vertical internal grid lines.
fn build_vertical_lines(
    view_model: &TableViewModel,
    geometry: &TableGeometry,
    resolution: &CellResolution,
    range: CellRange,
    slice_bounds: Rectangle,
    lines: &mut Vec<TableSliceLine>,
)
{
    for boundary_column in range.first_column..range.last_column {
        let x = geometry.vertical_line_position(boundary_column);
        let mut segment_start = None;

        for row in range.first_row..=range.last_row {
            let separated = resolution.owner(row, boundary_column)
                != resolution.owner(row, boundary_column + 1);

            if separated {
                if segment_start.is_none() {
                    segment_start = Some(row);
                }
            } else if let Some(start_row) = segment_start.take() {
                push_vertical_segment(geometry, slice_bounds, x, start_row, row, lines);
            }

            if row == range.last_row {
                if let Some(start_row) = segment_start.take() {
                    push_vertical_segment(geometry, slice_bounds, x, start_row, row + 1, lines);
                }
            }
        }
    }
}

/// Builds horizontal internal grid lines.
fn build_horizontal_lines(
    view_model: &TableViewModel,
    geometry: &TableGeometry,
    resolution: &CellResolution,
    range: CellRange,
    slice_bounds: Rectangle,
    lines: &mut Vec<TableSliceLine>,
)
{
    for boundary_row in range.first_row..range.last_row {
        let y = geometry.horizontal_line_position(boundary_row);
        let mut segment_start = None;

        for column in range.first_column..=range.last_column {
            let separated = resolution.owner(boundary_row, column)
                != resolution.owner(boundary_row + 1, column);

            if separated {
                if segment_start.is_none() {
                    segment_start = Some(column);
                }
            } else if let Some(start_column) = segment_start.take() {
                push_horizontal_segment(geometry, slice_bounds, y, start_column, column, lines);
            }

            if column == range.last_column {
                if let Some(start_column) = segment_start.take() {
                    push_horizontal_segment(
                        geometry,
                        slice_bounds,
                        y,
                        start_column,
                        column + 1,
                        lines,
                    );
                }
            }
        }
    }
}

/// Appends one vertical grid-line segment.
fn push_vertical_segment(
    geometry: &TableGeometry,
    slice_bounds: Rectangle,
    x: f32,
    first_row: usize,
    last_row_exclusive: usize,
    lines: &mut Vec<TableSliceLine>,
)
{
    let y1 = geometry.row_start(first_row);
    let y2 = geometry.row_start(last_row_exclusive);
    lines.push(TableSliceLine {
        start: Point::new(x - slice_bounds.x, y1 - slice_bounds.y),
        end: Point::new(x - slice_bounds.x, y2 - slice_bounds.y),
        width: geometry.line_width(),
    });
}

/// Appends one horizontal grid-line segment.
fn push_horizontal_segment(
    geometry: &TableGeometry,
    slice_bounds: Rectangle,
    y: f32,
    first_column: usize,
    last_column_exclusive: usize,
    lines: &mut Vec<TableSliceLine>,
)
{
    let x1 = geometry.column_start(first_column);
    let x2 = geometry.column_start(last_column_exclusive);
    lines.push(TableSliceLine {
        start: Point::new(x1 - slice_bounds.x, y - slice_bounds.y),
        end: Point::new(x2 - slice_bounds.x, y - slice_bounds.y),
        width: geometry.line_width(),
    });
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::widgets::table::{
        table_style::{TableGridLine, TableStyle},
        view_model::{
            MergedTableCell, NormalTableCell, TableCell, TableCellMatrix, TableGeometryMatrix,
            TableViewModel,
        },
        viewport::TableSliceCellText,
    };

    fn mock_view_model() -> TableViewModel
    {
        TableViewModel::new(
            TableGeometryMatrix::new(
                vec![100.0, 100.0, 130.0, 100.0, 100.0, 100.0],
                vec![100.0, 100.0, 100.0, 130.0, 100.0],
            ),
            TableCellMatrix::new(vec![
                vec![
                    normal("(0,0)"),
                    normal("(0,1)"),
                    normal("(0,2)"),
                    normal("(0,3)"),
                    normal("(0,4)"),
                    normal("(0,5)"),
                ],
                vec![
                    normal("(1,0)"),
                    normal("(1,1)"),
                    normal("(1,2)"),
                    normal("(1,3)"),
                    normal("(1,4)"),
                    normal("(1,5)"),
                ],
                vec![
                    normal("(2,0)"),
                    normal("(2,1)"),
                    normal("(2,2)"),
                    merged(1, 3),
                    normal("(2,4)"),
                    normal("(2,5)"),
                ],
                vec![
                    normal("(3,0)"),
                    normal("(3,1)"),
                    normal("(3,2)"),
                    merged(3, 2),
                    normal("(3,4)"),
                    normal("(3,5)"),
                ],
                vec![
                    normal("(4,0)"),
                    normal("(4,1)"),
                    merged(3, 2),
                    merged(3, 2),
                    normal("(4,4)"),
                    normal("(4,5)"),
                ],
            ]),
        )
    }

    fn normal(label: &str) -> TableCell
    {
        TableCell::Normal(NormalTableCell::new(label))
    }

    fn merged(origin_row: usize, origin_column: usize) -> TableCell
    {
        TableCell::MergedToUpwards(MergedTableCell {
            origin_row,
            origin_column,
        })
    }

    fn visual_style() -> TableStyle
    {
        TableStyle {
            grid_line: TableGridLine::Visual(1.0),
            draw_outer_border: false,
        }
    }

    fn sized_style() -> TableStyle
    {
        TableStyle {
            grid_line: TableGridLine::Sized(1.0),
            draw_outer_border: false,
        }
    }

    fn viewport() -> Rectangle
    {
        Rectangle::new(15.0, 20.0, 600.0, 500.0)
    }

    #[test]
    fn mock_table_has_expected_geometry()
    {
        let view_model = mock_view_model();

        assert_eq!(view_model.columns_count(), 6,);

        assert_eq!(view_model.rows_count(), 5,);

        assert_eq!(view_model.width(), 630.0,);

        assert_eq!(view_model.height(), 530.0,);
    }

    #[test]
    fn visual_slice_has_expected_origin()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        assert_eq!(slice.origin, Point::new(15.0, 20.0,),);
    }

    #[test]
    fn visual_slice_contains_all_mock_texts()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        assert_eq!(slice.texts.len(), 26,);

        let labels = slice
            .texts
            .iter()
            .map(|text| text.text.as_str())
            .collect::<Vec<_>>();

        assert!(!labels.contains(&"(2,3)"));
        assert!(!labels.contains(&"(3,3)"));
        assert!(!labels.contains(&"(4,2)"));
        assert!(!labels.contains(&"(4,3)"));
    }

    #[test]
    fn first_text_uses_slice_coordinates()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        assert_eq!(
            slice.texts[0],
            TableSliceCellText {
                text: "(0,0)".into(),
                position: Point::new(0.0, 0.0,),
            },
        );
    }

    #[test]
    fn visual_grid_lines_use_logical_boundaries()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        let vertical = slice
            .lines
            .iter()
            .filter(|line| line.start.x == line.end.x)
            .collect::<Vec<_>>();

        assert_eq!(vertical.len(), 5,);

        assert_eq!(
            vertical[0],
            &TableSliceLine {
                start: Point::new(100.0, 0.0),
                end: Point::new(100.0, 530.0),
                width: 1.0,
            },
        );

        assert_eq!(
            vertical[1],
            &TableSliceLine {
                start: Point::new(200.0, 0.0),
                end: Point::new(200.0, 530.0),
                width: 1.0,
            },
        );

        assert_eq!(
            vertical[2],
            &TableSliceLine {
                start: Point::new(330.0, 0.0),
                end: Point::new(330.0, 300.0),
                width: 1.0,
            },
        );

        assert_eq!(
            vertical[3],
            &TableSliceLine {
                start: Point::new(430.0, 0.0),
                end: Point::new(430.0, 530.0),
                width: 1.0,
            },
        );

        assert_eq!(
            vertical[4],
            &TableSliceLine {
                start: Point::new(530.0, 0.0),
                end: Point::new(530.0, 530.0),
                width: 1.0,
            },
        );
    }

    #[test]
    fn visual_merge_removes_internal_horizontal_lines()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        let horizontal = slice
            .lines
            .iter()
            .filter(|line| line.start.y == line.end.y)
            .collect::<Vec<_>>();

        assert_eq!(horizontal.len(), 6,);

        assert_eq!(
            horizontal[0],
            &TableSliceLine {
                start: Point::new(0.0, 100.0),
                end: Point::new(630.0, 100.0),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[1],
            &TableSliceLine {
                start: Point::new(0.0, 200.0),
                end: Point::new(330.0, 200.0),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[2],
            &TableSliceLine {
                start: Point::new(430.0, 200.0),
                end: Point::new(630.0, 200.0),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[3],
            &TableSliceLine {
                start: Point::new(0.0, 300.0),
                end: Point::new(630.0, 300.0),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[4],
            &TableSliceLine {
                start: Point::new(0.0, 430.0),
                end: Point::new(200.0, 430.0),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[5],
            &TableSliceLine {
                start: Point::new(330.0, 430.0),
                end: Point::new(630.0, 430.0),
                width: 1.0,
            },
        );
    }

    #[test]
    fn visual_slice_has_expected_line_count()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        assert_eq!(slice.lines.len(), 11,);
    }

    #[test]
    fn sized_geometry_occupies_grid_space()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &sized_style(), viewport());

        let vertical = slice
            .lines
            .iter()
            .filter(|line| line.start.x == line.end.x)
            .collect::<Vec<_>>();

        assert_eq!(vertical.len(), 5,);

        assert_eq!(vertical[0].start.x, 100.5,);

        assert_eq!(vertical[1].start.x, 201.5,);

        assert_eq!(vertical[2].start.x, 332.5,);

        assert_eq!(vertical[3].start.x, 433.5,);

        assert_eq!(vertical[4].start.x, 534.5,);
    }

    #[test]
    fn sized_merge_contains_grid_space()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &sized_style(), viewport());

        let horizontal = slice
            .lines
            .iter()
            .filter(|line| line.start.y == line.end.y)
            .collect::<Vec<_>>();

        assert_eq!(
            horizontal[1],
            &TableSliceLine {
                start: Point::new(0.0, 201.5),
                end: Point::new(333.0, 201.5),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[2],
            &TableSliceLine {
                start: Point::new(434.0, 201.5),
                end: Point::new(635.0, 201.5),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[4],
            &TableSliceLine {
                start: Point::new(0.0, 433.5),
                end: Point::new(202.0, 433.5),
                width: 1.0,
            },
        );

        assert_eq!(
            horizontal[5],
            &TableSliceLine {
                start: Point::new(333.0, 433.5),
                end: Point::new(635.0, 433.5),
                width: 1.0,
            },
        );
    }

    #[test]
    fn sized_geometry_has_expected_total_layout_size()
    {
        let view_model = mock_view_model();

        let geometry = TableGeometry::new(&view_model, TableGridLine::Sized(1.0));

        let range = CellRange::new(0, 4, 0, 5);

        assert_eq!(
            geometry.bounds(range),
            Rectangle::new(0.0, 0.0, 635.0, 534.0,),
        );
    }

    #[test]
    fn outer_border_is_not_generated()
    {
        let style = TableStyle {
            grid_line: TableGridLine::Visual(1.0),
            draw_outer_border: true,
        };

        let slice = ViewModelParser::slice(&mock_view_model(), &style, viewport());

        assert!(slice
            .lines
            .iter()
            .all(|line| { line.start.x != 0.0 || line.end.x != 0.0 }));

        assert!(slice
            .lines
            .iter()
            .all(|line| { line.start.y != 0.0 || line.end.y != 0.0 }));

        assert!(slice
            .lines
            .iter()
            .all(|line| { line.start.x != 630.0 || line.end.x != 630.0 }));

        assert!(slice
            .lines
            .iter()
            .all(|line| { line.start.y != 530.0 || line.end.y != 530.0 }));
    }

    #[test]
    fn backgrounds_are_currently_empty()
    {
        let slice = ViewModelParser::slice(&mock_view_model(), &visual_style(), viewport());

        assert!(slice.backgrounds.is_empty());
    }

    #[test]
    fn empty_viewport_produces_empty_slice()
    {
        let slice = ViewModelParser::slice(
            &mock_view_model(),
            &visual_style(),
            Rectangle::new(0.0, 0.0, 0.0, 500.0),
        );

        assert_eq!(slice.origin, Point::new(0.0, 0.0,),);

        assert!(slice.lines.is_empty());

        assert!(slice.texts.is_empty());

        assert!(slice.backgrounds.is_empty());
    }

    #[test]
    fn empty_table_produces_empty_slice()
    {
        let view_model = TableViewModel::new(
            TableGeometryMatrix::new(Vec::new(), Vec::new()),
            TableCellMatrix::empty(),
        );

        let slice = ViewModelParser::slice(&view_model, &visual_style(), viewport());

        assert_eq!(slice.origin, Point::new(0.0, 0.0,),);

        assert!(slice.lines.is_empty());

        assert!(slice.texts.is_empty());

        assert!(slice.backgrounds.is_empty());
    }
}
