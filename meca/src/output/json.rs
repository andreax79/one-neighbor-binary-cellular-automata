use crate::config::Configuration;
use crate::output::Output;
use crate::row::Row;
use anyhow::Result;
use serde_json::json;

// Output the rows to a JSON file
pub struct JSONOutput<'a> {
    config: &'a dyn Configuration,
    rows: Vec<Vec<usize>>,
}

impl JSONOutput<'_> {
    pub fn new<'a>(config: &'a dyn Configuration) -> JSONOutput<'a> {
        JSONOutput {
            config: config,
            rows: Vec::new(),
        }
    }
}

impl Output for JSONOutput<'_> {
    /// Draw the row on the image
    fn add_row(&mut self, row: &Row) {
        self.rows.push(
            row.cells
                .iter()
                .map(|cell| if cell.state { 1 } else { 0 })
                .collect(),
        );
    }

    /// Save the JSON
    fn close(&mut self) -> Result<()> {
        let out = json!({
            "config": {
                "rule": self.config.get_rule(0).to_string(),
                "size": self.config.get_size(),
                "alpha": self.config.get_alpha(0),
                "boundary": self.config.get_boundaries().to_string(),
                "update_pattern": self.config.get_update_pattern().to_string(),
                "initial_state": self.config.get_initial_state().to_string(),
                "seed": self.config.get_seed(),
                "steps": self.config.get_steps(),
                "update_order": self.config.get_update_order(),
            },
            "rows": self.rows,
        });
        println!("{}", out);
        Ok(())
    }
}
