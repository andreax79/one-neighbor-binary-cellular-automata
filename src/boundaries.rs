use crate::cell::Cell;
use std::fmt;
use std::str;

#[derive(Clone, Copy, PartialEq)]
pub enum Boundaries {
    // Boundary Conditions
    //              +---------------------+
    // Periodic   9 | 0 1 2 3 4 5 6 7 8 9 | 0
    //              +---------------------+
    Periodic,
    //              +---------------------+
    // Fixed      X | 0 1 2 3 4 5 6 7 8 9 | X
    //              +---------------------+
    Fixed(bool),
    //              +---------------------+
    // Adiabatic  0 | 0 1 2 3 4 5 6 7 8 9 | 9
    //              +---------------------+
    Adiabatic,
    //              +---------------------+
    // Reflective 1 | 0 1 2 3 4 5 6 7 8 9 | 8
    //              +---------------------+
    Reflective,
}

impl Boundaries {
    /// Get the state of the cell at index i
    pub fn get_state(&self, cells: &Vec<Cell>, i: isize) -> bool {
        let index = match self {
            Boundaries::Periodic => (i + cells.len() as isize) as usize % cells.len(),
            Boundaries::Adiabatic => {
                if i < 0 {
                    0 as usize
                } else if i >= cells.len() as isize {
                    cells.len() - 1
                } else {
                    i as usize
                }
            }
            Boundaries::Reflective => {
                if i < 0 {
                    (-i) as usize
                } else if i >= cells.len() as isize {
                    2 * cells.len() - i as usize - 2
                } else {
                    i as usize
                }
            }
            Boundaries::Fixed(fixed) => {
                if i < 0 || i >= cells.len() as isize {
                    return *fixed;
                }
                i as usize
            }
        };
        cells[index].state
    }
}

impl str::FromStr for Boundaries {
    type Err = &'static str;

    /// Implement the FromStr trait for Boundaries to parse the boundary type
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "periodic" => Ok(Boundaries::Periodic),
            "fixedon" => Ok(Boundaries::Fixed(true)),
            "fixedoff" => Ok(Boundaries::Fixed(false)),
            "adiabatic" => Ok(Boundaries::Adiabatic),
            "reflective" => Ok(Boundaries::Reflective),
            _ => Err("Invalid boundary type: must be 'Periodic', 'FixedOn', 'FixedOff', 'Adiabatic', or 'Reflective'"),
        }
    }
}

impl fmt::Display for Boundaries {
    /// Implement the Display trait for Boundaries to print the boundary type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Boundaries::Periodic => write!(f, "Periodic"),
            Boundaries::Fixed(state) => write!(f, "Fixed{}", if *state { "On" } else { "Off" }),
            Boundaries::Adiabatic => write!(f, "Adiabatic"),
            Boundaries::Reflective => write!(f, "Reflective"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Rule;

    fn test_cells() -> Vec<Cell> {
        let rule = Rule { number: 0 };
        vec![
            Cell::new(rule, 0.0, false),
            Cell::new(rule, 0.0, true),
            Cell::new(rule, 0.0, false),
        ]
    }

    #[test]
    fn test_periodic_boundary() {
        let cells = test_cells();
        let boundary = Boundaries::Periodic;

        assert_eq!(boundary.get_state(&cells, -1), false); // Wraps around to last cell
        assert_eq!(boundary.get_state(&cells, cells.len() as isize), false); // Wraps around to first cell
        assert_eq!(boundary.get_state(&cells, 1), true);
    }

    #[test]
    fn test_adiabatic_boundary() {
        let cells = test_cells();
        let boundary = Boundaries::Adiabatic;

        assert_eq!(boundary.get_state(&cells, -1), false);
        assert_eq!(boundary.get_state(&cells, cells.len() as isize), false);
        assert_eq!(boundary.get_state(&cells, 1), true);
    }

    #[test]
    fn test_reflective_boundary() {
        let cells = test_cells();
        let boundary = Boundaries::Reflective;

        assert_eq!(boundary.get_state(&cells, -1), true);
        assert_eq!(boundary.get_state(&cells, cells.len() as isize), true);
        assert_eq!(boundary.get_state(&cells, 1), true);
    }

    #[test]
    fn test_fixed_boundary() {
        let cells = test_cells();
        let fixed_true = Boundaries::Fixed(true);
        let fixed_false = Boundaries::Fixed(false);

        assert_eq!(fixed_true.get_state(&cells, -1), true);
        assert_eq!(fixed_true.get_state(&cells, 30), true);
        assert_eq!(fixed_false.get_state(&cells, -1), false);
        assert_eq!(fixed_false.get_state(&cells, 30), false);
    }
}
