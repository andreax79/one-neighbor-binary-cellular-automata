use crate::boundaries::Boundaries;
use crate::cell::Cell;
use crate::rule::Rule;
use anyhow::Result;
use std::fmt;

/// One Neighbor Binary Cellular Automata (1nCA)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OnCARule {
    pub number: u8, // Only allows values 0-15
}

impl Rule for OnCARule {
    /// Step function for the rule
    fn step(
        &self,
        cells: &Vec<Cell>,
        i: usize,
        time: usize,
        boundaries: &Boundaries,
    ) -> (bool, bool, bool) {
        let cell_state = boundaries.get_state(cells, i as isize);
        let n = neighbor_index(i, time);
        let neighbor_state = boundaries.get_state(cells, n);
        let new_state = self.compute(cell_state, neighbor_state);
        (new_state, neighbor_state, false)
    }
}

fn neighbor_index(i: usize, time: usize) -> isize {
    let offset = if time % 2 == 1 { 1 } else { -1 };
    i as isize + offset
}

impl OnCARule {
    pub fn new(number: u8) -> Result<Self> {
        if number > 15 {
            return Err(anyhow::anyhow!(
                "Invalid rule number: must be between 0 and 15"
            ));
        }
        Ok(Self { number })
    }

    pub fn compute(&self, self_cell: bool, neighbor_cell: bool) -> bool {
        // Compute the new state of the cell
        match self.number {
            0 => false,
            1 => !(self_cell || neighbor_cell),
            2 => !self_cell && neighbor_cell,
            3 => !self_cell,
            4 => self_cell && !neighbor_cell,
            5 => !neighbor_cell,
            6 => self_cell ^ neighbor_cell,
            7 => !(self_cell & neighbor_cell),
            8 => self_cell & neighbor_cell,
            9 => !(self_cell ^ neighbor_cell),
            10 => neighbor_cell,
            11 => !(self_cell & !neighbor_cell),
            12 => self_cell,
            13 => !(!self_cell & neighbor_cell),
            14 => self_cell || neighbor_cell,
            _ => true,
        }
    }

    pub fn get_sensitivity(&self) -> f64 {
        // Binder 1993, Binder 1994
        let args = [
            ((false, false), (false, true)),
            ((false, false), (true, false)),
            ((false, true), (false, false)),
            ((false, true), (true, true)),
            ((true, false), (true, true)),
            ((true, false), (false, false)),
            ((true, true), (true, false)),
            ((true, true), (false, true)),
        ];

        let count = args
            .iter()
            .filter(|&&(a, b)| self.compute(a.0, a.1) != self.compute(b.0, b.1))
            .count() as f64;
        count / (args.len() as f64)
    }
}

impl fmt::Display for OnCARule {
    /// Implement the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
