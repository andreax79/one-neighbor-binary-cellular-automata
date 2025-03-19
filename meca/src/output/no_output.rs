use crate::output::Output;
use crate::row::Row;
use anyhow::Result;

// Suppress output
pub struct NoOutput {}

impl NoOutput {
    pub fn new() -> NoOutput {
        NoOutput {}
    }
}

impl Output for NoOutput {
    fn add_row(&mut self, _row: &Row) {}

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
