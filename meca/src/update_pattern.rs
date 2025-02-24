use crate::cell::Cell;
use crate::config::Configuration;
use crate::rng::generate_random_order_indexes;
use crate::rng::random_range;
use crate::row::Row;
use anyhow::Result;
use rand_core::RngCore;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdatePattern {
    Synchronous,                 // all cells updated in parallel
    RasRandomIndependent(usize), // n random cells updated per step
    RasRandomOrder,              // all nodes updated, but in random order
    OasCyclic,                   // updates follow a predefined order
    OasEqClocked(usize),         // updates follow a predefined order, with a clock period
}

impl UpdatePattern {
    pub const VALID_VALUES: [&'static str; 5] = [
        "Synchronous",
        "RasRandomIndependent<n>",
        "RasRandomOrder",
        "OasCyclic",
        "OasEqClocked<n>",
    ];

    /// Returns a comma-separated list of valid update pattern names
    pub fn valid_values() -> String {
        Self::VALID_VALUES.join(", ")
    }
}

impl UpdatePattern {
    /// Update the row of cells according to the update pattern
    pub fn update(self, row: &Row, rng: &mut dyn RngCore, config: &dyn Configuration) -> Vec<Cell> {
        match self {
            UpdatePattern::RasRandomIndependent(n) => {
                self.ras_random_independent_update(row, n, rng, config)
            }
            UpdatePattern::RasRandomOrder => self.ras_random_order_update(row, rng, config),
            UpdatePattern::OasCyclic => self.oas_cyclic_update(row, config),
            UpdatePattern::OasEqClocked(eq_clocked_times) => {
                self.oas_eq_clocked_update(row, eq_clocked_times, config)
            }
            _ => self.synchronous_update(row, config),
        }
    }

    /// At each time step, n cells to update are chosen at random
    fn ras_random_independent_update(
        &self,
        row: &Row,
        n: usize,
        rng: &mut dyn RngCore,
        config: &dyn Configuration,
    ) -> Vec<Cell> {
        let mut new_cells = row.cells.clone();

        for _ in 0..n {
            // Generate a random index
            let i = random_range(0..row.cells.len(), rng);
            new_cells[i] = row.cells[i].update(&new_cells, i, row.t, config);
        }

        new_cells
    }

    /// At each time step, all nodes are updated, but in random order
    fn ras_random_order_update(
        self,
        row: &Row,
        rng: &mut dyn RngCore,
        config: &dyn Configuration,
    ) -> Vec<Cell> {
        let mut new_cells = row.cells.clone();

        for &i in generate_random_order_indexes(row.get_size(), rng).iter() {
            new_cells[i] = row.cells[i].update(&new_cells, i, row.t, config);
        }

        new_cells
    }

    /// At each time step, all nodes are updated in a predefined order
    fn oas_cyclic_update(self, row: &Row, config: &dyn Configuration) -> Vec<Cell> {
        let mut new_cells = row.cells.clone();

        for &i in config.get_update_order().unwrap().iter() {
            new_cells[i] = row.cells[i].update(&new_cells, i, row.t, config);
        }

        new_cells
    }

    /// At each time step, all nodes are updated in a predefined order, with a clock period
    fn oas_eq_clocked_update(
        self,
        row: &Row,
        eq_clocked_times: usize,
        config: &dyn Configuration,
    ) -> Vec<Cell> {
        let mut new_cells = row.cells.clone();

        for j in 0..eq_clocked_times {
            for i in 0..row.get_size() {
                if config.get_update_order().unwrap()[i] == j {
                    new_cells[i] = row.cells[i].update(&new_cells, i, row.t, config);
                }
            }
        }

        new_cells
    }

    /// All cells are updated in parallel at each time step
    fn synchronous_update(self, row: &Row, config: &dyn Configuration) -> Vec<Cell> {
        row.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| cell.update(&row.cells, i, row.t, config))
            .collect()
    }
}

impl FromStr for UpdatePattern {
    type Err = anyhow::Error;

    /// Parse the update pattern from a string
    fn from_str(s: &str) -> Result<Self> {
        // Split the string into a prefix and a number
        let (prefix, num_str) = s.split_at(s.find(|c: char| c.is_digit(10)).unwrap_or(s.len()));
        let num = if num_str.is_empty() {
            1 as usize
        } else {
            num_str.parse::<usize>().unwrap()
        };

        match prefix.to_lowercase().as_str() {
            "synchronous" => Ok(UpdatePattern::Synchronous),
            "rasrandomindependent" => Ok(UpdatePattern::RasRandomIndependent(num)),
            "rasrandomorder" => Ok(UpdatePattern::RasRandomOrder),
            "oascyclic" => Ok(UpdatePattern::OasCyclic),
            "oaseqclocked" => Ok(UpdatePattern::OasEqClocked(num)),
            _ => Err(anyhow::anyhow!(
                "Invalid update pattern. Valid patterns are: {}",
                Self::valid_values()
            )),
        }
    }
}

impl fmt::Display for UpdatePattern {
    /// Implement the Display trait for UpdatePattern to print the update pattern
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdatePattern::Synchronous => write!(f, "Synchronous"),
            UpdatePattern::RasRandomIndependent(t) => write!(f, "RasRandomIndependent{}", t),
            UpdatePattern::RasRandomOrder => write!(f, "RasRandomOrder"),
            UpdatePattern::OasCyclic => write!(f, "OasCyclic"),
            UpdatePattern::OasEqClocked(t) => write!(f, "OasEqClocked{}", t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            "Synchronous".parse::<UpdatePattern>().unwrap(),
            UpdatePattern::Synchronous
        );
        assert_eq!(
            "RasRandomIndependent5".parse::<UpdatePattern>().unwrap(),
            UpdatePattern::RasRandomIndependent(5)
        );
        assert_eq!(
            "RasRandomOrder".parse::<UpdatePattern>().unwrap(),
            UpdatePattern::RasRandomOrder
        );
        assert_eq!(
            "OasCyclic".parse::<UpdatePattern>().unwrap(),
            UpdatePattern::OasCyclic
        );
        assert_eq!(
            "OasEqClocked10".parse::<UpdatePattern>().unwrap(),
            UpdatePattern::OasEqClocked(10)
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(UpdatePattern::from_str("InvalidPattern").is_err());
        assert!(UpdatePattern::from_str("RasRandomIndependentXYZ").is_err()); // Invalid number
        assert!(UpdatePattern::from_str("OasEqClocked-5").is_err()); // Negative numbers are not handled
    }

    #[test]
    fn test_display() {
        assert_eq!(UpdatePattern::Synchronous.to_string(), "Synchronous");
        assert_eq!(
            UpdatePattern::RasRandomIndependent(3).to_string(),
            "RasRandomIndependent3"
        );
        assert_eq!(UpdatePattern::RasRandomOrder.to_string(), "RasRandomOrder");
        assert_eq!(UpdatePattern::OasCyclic.to_string(), "OasCyclic");
        assert_eq!(UpdatePattern::OasEqClocked(7).to_string(), "OasEqClocked7");
    }
}
