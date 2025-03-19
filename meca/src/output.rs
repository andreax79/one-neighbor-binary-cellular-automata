use crate::color_scheme::ColorScheme;
use crate::config::Configuration;
use crate::output::console::ConsoleOutput;
use crate::output::image::{ImageOutput, DEFAULT_CELL_SIZE};
use crate::output::json::JSONOutput;
use crate::output::no_output::NoOutput;
use crate::row::Row;
use anyhow::Result;
use std::fmt;
use std::str;

pub mod console;
pub mod image;
pub mod json;
pub mod no_output;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    NoOutput,
    Console,
    CSV,
    JSON,
    TimeSpace(usize),      // (Cell size)
    TimeSpaceGraph(usize), // (Cell size)
}

impl OutputType {
    pub const VALID_VALUES: [&'static str; 6] = [
        "TimeSpaceGraph",
        "TimeSpace",
        "Console",
        "CSV",
        "JSON",
        "None",
    ];

    /// Returns a comma-separated list of valid update pattern names
    pub fn valid_values() -> String {
        Self::VALID_VALUES.join(", ")
    }

    pub fn open_output<'a>(
        &self,
        config: &'a dyn Configuration,
        filename: &str,
        color_scheme: ColorScheme,
    ) -> Box<dyn Output + 'a> {
        match self {
            OutputType::NoOutput => Box::new(NoOutput::new()),
            OutputType::Console | OutputType::CSV => Box::new(ConsoleOutput::new(*self)),
            OutputType::JSON => Box::new(JSONOutput::new(config)),
            OutputType::TimeSpace(_) | OutputType::TimeSpaceGraph(_) => {
                Box::new(ImageOutput::new(config, filename, color_scheme, *self))
            }
        }
    }
}

impl str::FromStr for OutputType {
    type Err = anyhow::Error;

    /// Parse the output type from a string
    fn from_str(s: &str) -> Result<Self> {
        // Split the string into a prefix and a number
        let (prefix, num_str) = s.split_at(s.find(|c: char| c.is_digit(10)).unwrap_or(s.len()));
        let cell_size = if num_str.is_empty() {
            DEFAULT_CELL_SIZE
        } else {
            num_str.parse::<usize>().unwrap()
        };

        match prefix.to_lowercase().as_str() {
            "none" => Ok(OutputType::NoOutput),
            "console" => Ok(OutputType::Console),
            "csv" => Ok(OutputType::CSV),
            "json" => Ok(OutputType::JSON),
            "timespace" => Ok(OutputType::TimeSpace(cell_size)),
            "timespacegraph" => Ok(OutputType::TimeSpaceGraph(cell_size)),
            _ => Err(anyhow::anyhow!("Invalid output option: must be 'TimeSpaceGraph', 'TimeSpace', 'Console', or 'None'")),
        }
    }
}

impl fmt::Display for OutputType {
    /// Implement the Display trait for OutputType to print the output type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputType::NoOutput => write!(f, "None"),
            OutputType::Console => write!(f, "Console"),
            OutputType::CSV => write!(f, "CSV"),
            OutputType::JSON => write!(f, "JSON"),
            OutputType::TimeSpace(cell_size) => {
                if *cell_size == DEFAULT_CELL_SIZE {
                    write!(f, "TimeSpace")
                } else {
                    write!(f, "TimeSpace{}", cell_size)
                }
            }
            OutputType::TimeSpaceGraph(cell_size) => {
                if *cell_size == DEFAULT_CELL_SIZE {
                    write!(f, "TimeSpaceGraph")
                } else {
                    write!(f, "TimeSpaceGraph{}", cell_size)
                }
            }
        }
    }
}

pub trait Output {
    fn add_row(&mut self, row: &Row);
    fn close(&mut self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        assert!(matches!(
            "none".parse::<OutputType>(),
            Ok(OutputType::NoOutput)
        ));
        assert!(matches!(
            "console".parse::<OutputType>(),
            Ok(OutputType::Console)
        ));
        assert!(matches!("CSV".parse::<OutputType>(), Ok(OutputType::CSV)));
        assert!(matches!("JSON".parse::<OutputType>(), Ok(OutputType::JSON)));
        assert!(matches!(
            "timespace".parse::<OutputType>(),
            Ok(OutputType::TimeSpace(DEFAULT_CELL_SIZE))
        ));
        assert!(matches!(
            "timespace10".parse::<OutputType>(),
            Ok(OutputType::TimeSpace(10))
        ));
        assert!(matches!(
            "timespacegraph".parse::<OutputType>(),
            Ok(OutputType::TimeSpaceGraph(DEFAULT_CELL_SIZE))
        ));
        assert!(matches!(
            "timespacegraph20".parse::<OutputType>(),
            Ok(OutputType::TimeSpaceGraph(20))
        ));
    }

    #[test]
    fn test_parse_invalid() {
        assert!("invalid".parse::<OutputType>().is_err());
        assert!("timespaceXYZ".parse::<OutputType>().is_err());
        assert!("timespacegraph-5".parse::<OutputType>().is_err());
    }

    #[test]
    fn test_display() {
        assert!(OutputType::NoOutput.to_string() == "None");
        assert!(OutputType::Console.to_string() == "Console");
        assert!(OutputType::CSV.to_string() == "CSV");
        assert!(OutputType::JSON.to_string() == "JSON");
        assert!(OutputType::TimeSpace(DEFAULT_CELL_SIZE).to_string() == "TimeSpace");
        assert!(OutputType::TimeSpace(10).to_string() == "TimeSpace10");
        assert!(OutputType::TimeSpaceGraph(DEFAULT_CELL_SIZE).to_string() == "TimeSpaceGraph");
        assert!(OutputType::TimeSpaceGraph(20).to_string() == "TimeSpaceGraph20");
    }
}
