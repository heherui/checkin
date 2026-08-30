# Table Widget

## View Model
```
                   TableViewModel
                         │
             ┌───────────┴───────────┐
             ↓                       ↓
    TableGeometryMatrix       TableCellMatrix
             │                       │
       row/column size          logical cells
                                     │
                         ┌───────────┼───────────┐
                         ↓           ↓           ↓
                      Normal      Merged      Merged
                                    ↑           ↑
                                    └──── origin
                         
                         ↓
                 TableViewCache
                         ↓
                  TableSlice
                         ↓
                     Render
```
## Viewport
[`TableSlice`] is table data in render ready structure.