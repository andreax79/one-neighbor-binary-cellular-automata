use crate::boundaries::Boundaries;
use crate::cell::Cell;
use crate::rule::Rule;
use anyhow::Result;
use std::fmt;

/// Elementary Cellular Automaton
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ECARule {
    /// Rule number
    pub number: u8,
}

impl Rule for ECARule {
    /// Step function for the rule
    fn step(
        &self,
        cells: &Vec<Cell>,
        i: usize,
        _time: usize,
        boundaries: &Boundaries,
    ) -> (bool, bool, bool) {
        let left_state = boundaries.get_state(cells, i as isize - 1);
        let cell_state = boundaries.get_state(cells, i as isize);
        let right_state = boundaries.get_state(cells, i as isize + 1);
        let n = (right_state as u8) | ((cell_state as u8) << 1) | ((left_state as u8) << 2);
        let new_state = (self.number >> n) & 1 == 1;
        (new_state, left_state, right_state)
    }
}

impl ECARule {
    pub fn new(number: u8) -> Result<Self> {
        Ok(Self { number })
    }
}

impl fmt::Display for ECARule {
    /// Implement the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ECA {}", self.number)
    }
}
