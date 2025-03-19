use crate::output::Output;
use crate::output::OutputType;
use crate::row::Row;
use anyhow::Result;

/// Print the row to the console
pub struct ConsoleOutput {
    format_fun: fn(&Row) -> String,
}

/// Format a row as CSV
fn csv_format(row: &Row) -> String {
    row.cells
        .iter()
        .map(|cell| if cell.state { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join(",")
}

/// Format a row as a string of 'O' and '.' characters
fn console_format(row: &Row) -> String {
    row.cells
        .iter()
        .map(|cell| if cell.state { 'O' } else { '.' })
        .collect()
}

impl ConsoleOutput {
    pub fn new(output_type: OutputType) -> ConsoleOutput {
        ConsoleOutput {
            format_fun: if let OutputType::CSV = output_type {
                csv_format
            } else {
                console_format
            },
        }
    }
}

impl Output for ConsoleOutput {
    /// Print the row to the console
    fn add_row(&mut self, row: &Row) {
        println!("{}", (self.format_fun)(row));
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
