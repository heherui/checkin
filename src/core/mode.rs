/// High-level interaction mode for the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    CheckIn,
    Edit,
}
