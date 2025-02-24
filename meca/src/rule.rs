use crate::boundaries::Boundaries;
use crate::cell::Cell;
use anyhow::Result;
use std::fmt;
use std::str::FromStr;

pub mod eca; // Elementary Cellular Automata
pub mod onca; // One Neighbor Cellular Automata

pub trait Rule: fmt::Display {
    /// Step function for the rule
    fn step(
        &self,
        cells: &Vec<Cell>,
        i: usize,
        time: usize,
        boundaries: &Boundaries,
    ) -> (bool, bool, bool);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    ONCA,
    ECA,
}

impl RuleType {
    pub const VALID_VALUES: [&'static str; 2] = ["1nCA", "ECA"];

    /// Returns a comma-separated list of valid rule names
    pub fn valid_values() -> String {
        Self::VALID_VALUES.join(", ")
    }

    /// Get the rule for the given number
    pub fn get_rule(self, number: u8) -> Result<Box<dyn Rule>> {
        match self {
            RuleType::ONCA => {
                onca::OnCARule::new(number).map(|rule| Box::new(rule) as Box<dyn Rule>)
            }
            RuleType::ECA => eca::ECARule::new(number).map(|rule| Box::new(rule) as Box<dyn Rule>),
        }
    }
}

impl FromStr for RuleType {
    type Err = anyhow::Error;

    /// Implement the FromStr trait for RuleType to parse the rule
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "1nca" => Ok(RuleType::ONCA),
            "eca" => Ok(RuleType::ECA),
            _ => Err(anyhow::anyhow!(
                "Invalid rule type. Valid types are: {}",
                Self::valid_values()
            )),
        }
    }
}

impl fmt::Display for RuleType {
    /// Implement the Display trait for RuleType to print the rule type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleType::ONCA => write!(f, "1nCA"),
            RuleType::ECA => write!(f, "ECA"),
        }
    }
}
