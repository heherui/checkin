use crate::widgets::table::{
    view_model::{
        TableCell,
        TableViewModel,
    },
    viewport::{
        Point,
        Rectangle,
        TableSliceCellText,
    },
};

use super::geometry::{
    CellRange,
    TableGeometry,
};

/// A logical coordinate identifying the owner of a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CellCoordinate
{
    pub row: usize,
    pub column: usize,
}

/// The complete resolved ownership information of a table.
///
/// Every logical cell points to the normal cell that owns its content.
/// Merged cells therefore become ordinary references to their owner.
#[derive(Debug, Clone)]
pub(super) struct CellResolution
{
    owners: Vec<Vec<Option<CellCoordinate>>>,
    regions: Vec<Vec<Option<CellRange>>>,
}

impl CellResolution
{
    /// Resolves all logical cell ownership and merged regions.
    pub fn new(view_model: &TableViewModel) -> Self
    {
        let rows = view_model.rows_count();
        let columns = view_model.columns_count();

        let mut owners =
            vec![vec![None; columns]; rows];

        for row in 0..rows
        {
            for column in 0..columns
            {
                owners[row][column] =
                    resolve_owner(
                        view_model,
                        row,
                        column,
                    );
            }
        }

        let mut regions =
            vec![vec![None; columns]; rows];

        for row in 0..rows
        {
            for column in 0..columns
            {
                let Some(owner) = owners[row][column]
                else {
                    continue;
                };

                let region =
                    regions[owner.row][owner.column]
                        .get_or_insert(
                            CellRange::new(
                                owner.row,
                                owner.row,
                                owner.column,
                                owner.column,
                            ),
                        );

                region.first_row =
                    region.first_row.min(row);

                region.last_row =
                    region.last_row.max(row);

                region.first_column =
                    region.first_column.min(column);

                region.last_column =
                    region.last_column.max(column);
            }
        }

        for row in 0..rows
        {
            for column in 0..columns
            {
                let Some(owner) = owners[row][column]
                else {
                    continue;
                };

                regions[row][column] =
                    regions[owner.row][owner.column];
            }
        }

        Self {
            owners,
            regions,
        }
    }

    /// Returns the resolved owner of a logical cell.
    pub fn owner(
        &self,
        row: usize,
        column: usize,
    ) -> Option<CellCoordinate>
    {
        self.owners
            .get(row)?
            .get(column)?
            .as_ref()
            .copied()
    }

    /// Returns the complete merged region of a cell.
    pub fn region(
        &self,
        row: usize,
        column: usize,
    ) -> Option<CellRange>
    {
        self.regions
            .get(row)?
            .get(column)?
            .as_ref()
            .copied()
    }

    /// Expands a slice range so that every merged cell intersecting it is
    /// represented in its entirety.
    ///
    /// A merged cell may begin outside the viewport but still intersect it.
    /// Such a cell must not be truncated because its renderer-visible
    /// geometry belongs to one logical cell.
    pub fn expand_slice(
        &self,
        range: &mut CellRange,
    )
    {
        loop
        {
            let previous = *range;

            for row in range.first_row..=range.last_row
            {
                for column in range.first_column..=range.last_column
                {
                    let Some(region) =
                        self.region(row, column)
                    else {
                        continue;
                    };

                    range.first_row =
                        range.first_row.min(region.first_row);

                    range.last_row =
                        range.last_row.max(region.last_row);

                    range.first_column =
                        range.first_column.min(region.first_column);

                    range.last_column =
                        range.last_column.max(region.last_column);
                }
            }

            if *range == previous
            {
                break;
            }
        }
    }
}

/// A cell that is ready for the text-producing stage of the parser.
#[derive(Debug, Clone)]
pub(super) struct ResolvedCell
{
    pub bounds: Rectangle,
    pub label: String,
}

/// Resolves all renderable cells inside a slice range.
///
/// Merged cells are emitted exactly once, from their owning normal cell.
pub fn resolve_cells(
    view_model: &TableViewModel,
    geometry: &TableGeometry,
    resolution: &CellResolution,
    range: CellRange,
    slice_bounds: Rectangle,
) -> Vec<ResolvedCell>
{
    let mut cells = Vec::new();

    for row in range.first_row..=range.last_row
    {
        for column in range.first_column..=range.last_column
        {
            let Some(owner) =
                resolution.owner(row, column)
            else {
                continue;
            };

            if owner.row != row
                || owner.column != column
            {
                continue;
            }

            let Some(region) =
                resolution.region(row, column)
            else {
                continue;
            };

            let Some(TableCell::Normal(cell)) =
                view_model.cell_matrix.get(
                    owner.row,
                    owner.column,
                )
            else {
                continue;
            };

            let table_bounds =
                geometry.bounds(region);

            let bounds = Rectangle::new(
                table_bounds.x - slice_bounds.x,
                table_bounds.y - slice_bounds.y,
                table_bounds.width,
                table_bounds.height,
            );

            cells.push(
                ResolvedCell {
                    bounds,
                    label: cell.label.clone(),
                },
            );
        }
    }

    cells
}

/// Converts resolved cells into slice text elements.
pub fn build_texts(
    cells: &[ResolvedCell],
) -> Vec<TableSliceCellText>
{
    cells
        .iter()
        .filter(|cell| !cell.label.is_empty())
        .map(|cell| TableSliceCellText {
            text: cell.label.clone(),
            position: Point::new(
                cell.bounds.x,
                cell.bounds.y,
            ),
        })
        .collect()
}

/// Resolves the ultimate normal-cell owner of a logical cell.
fn resolve_owner(
    view_model: &TableViewModel,
    row: usize,
    column: usize,
) -> Option<CellCoordinate>
{
    let max_steps =
        view_model.rows_count()
            * view_model.columns_count()
            + 1;

    let mut current =
        CellCoordinate { row, column };

    for _ in 0..max_steps
    {
        let cell =
            view_model.cell_matrix.get(
                current.row,
                current.column,
            )?;

        match cell
        {
            TableCell::Normal(_) => {
                return Some(current);
            }

            TableCell::MergedToUpwards(cell)
            | TableCell::MergedToLeft(cell) => {
                current = CellCoordinate {
                    row: cell.origin_row,
                    column: cell.origin_column,
                };
            }
        }
    }

    None
}