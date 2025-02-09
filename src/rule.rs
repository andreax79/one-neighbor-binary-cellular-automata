use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    pub number: u8, // Only allows values 0-15
}

impl Rule {
    pub fn new(number: u8) -> Result<Self, &'static str> {
        if number > 15 {
            return Err("Invalid rule number: must be between 0 and 15");
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

impl fmt::Display for Rule {
    /// Implement the Display trait for Rule to print the rule number
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
