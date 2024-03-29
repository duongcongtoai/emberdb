use crate::storage::tile::LogicalTile;

pub mod insert;
pub mod seq_scan;
pub mod update;

/// TODO: still a dummy trait
pub trait Executor {
    fn execute(&self);
    fn get_output(&self) -> LogicalTile;
}
