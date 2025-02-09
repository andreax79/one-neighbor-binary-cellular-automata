use crate::activation::Activation;
use crate::cell::Cell;
use image::Rgb;
use std::fmt;
use std::str;

pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
pub const ORANGE: Rgb<u8> = Rgb([255, 200, 0]);
pub const CYAN: Rgb<u8> = Rgb([0, 255, 255]);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    BlackWhite,
    OmegaColor,
    ActivationColor,
}

// Compute the color of the cell based on the black and white color scheme
fn no_color(cell: &Cell) -> Rgb<u8> {
    if cell.state {
        BLACK
    } else {
        WHITE
    }
}

/// Compute the color of the cell based on the omega value
fn omega_color(cell: &Cell) -> Rgb<u8> {
    let c = (cell.omega / cell.big_omega * 255.0) as u8;
    if cell.state {
        Rgb([255 - c, 255 - c, 255 - c])
    } else {
        Rgb([255, 255, 255 - c])
    }
}

/// Compute the color of the cell based on the activation
fn activation_color(cell: &Cell) -> Rgb<u8> {
    match cell.activation {
        Activation::NewStateOff => WHITE,
        Activation::BothOn => BLACK,
        Activation::NeighborOn => GREEN,
        Activation::SelfOn => RED,
        Activation::BothOff => BLUE,
    }
}

impl ColorScheme {
    /// Get the color of the cell based on the color scheme
    pub fn get_color(&self, cell: &Cell) -> Rgb<u8> {
        match self {
            ColorScheme::BlackWhite => no_color(&cell),
            ColorScheme::OmegaColor => omega_color(&cell),
            ColorScheme::ActivationColor => activation_color(&cell),
        }
    }
}

impl str::FromStr for ColorScheme {
    type Err = &'static str;

    /// Implement the FromStr trait for ColorScheme to parse the color scheme
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blackwhite" => Ok(ColorScheme::BlackWhite),
            "omega" => Ok(ColorScheme::OmegaColor),
            "activation" => Ok(ColorScheme::ActivationColor),
            _ => Err("Invalid color scheme: must be 'BlackWhite', 'Omega', or 'Activation'"),
        }
    }
}

impl fmt::Display for ColorScheme {
    /// Implement the Display trait for ColorScheme to print the color scheme
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorScheme::BlackWhite => write!(f, "BlackWhite"),
            ColorScheme::OmegaColor => write!(f, "Omega"),
            ColorScheme::ActivationColor => write!(f, "Activation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Rule;

    #[test]
    fn test_parse_valid() {
        assert!(matches!(
            "blackwhite".parse::<ColorScheme>(),
            Ok(ColorScheme::BlackWhite)
        ));
        assert!(matches!(
            "omega".parse::<ColorScheme>(),
            Ok(ColorScheme::OmegaColor)
        ));
        assert!(matches!(
            "activation".parse::<ColorScheme>(),
            Ok(ColorScheme::ActivationColor)
        ));
    }

    #[test]
    fn test_parse_invalid() {
        assert!("invalid".parse::<ColorScheme>().is_err());
        assert!("blackwhite123".parse::<ColorScheme>().is_err());
        assert!("omega_color".parse::<ColorScheme>().is_err());
    }

    #[test]
    fn test_display() {
        assert!(ColorScheme::BlackWhite.to_string() == "BlackWhite");
        assert!(ColorScheme::OmegaColor.to_string() == "Omega");
        assert!(ColorScheme::ActivationColor.to_string() == "Activation");
    }

    #[test]
    fn test_blackwhite_color_scheme() {
        let rule = Rule::new(0).unwrap();
        let black_cell = Cell::new(rule, 0.0, true);
        let white_cell = Cell::new(rule, 0.0, false);

        assert!(ColorScheme::BlackWhite.get_color(&black_cell) == BLACK);
        assert!(ColorScheme::BlackWhite.get_color(&white_cell) == WHITE);
    }

    #[test]
    fn test_activation_color_scheme() {
        let rule = Rule::new(0).unwrap();
        let new_state_off = Cell {
            state: true,
            rule: rule,
            activation: Activation::NewStateOff,
            omega: 0.0,
            big_omega: 1.0,
            alpha: 0.0,
        };
        let both_on = Cell {
            state: true,
            rule: rule,
            activation: Activation::BothOn,
            omega: 0.0,
            big_omega: 1.0,
            alpha: 0.0,
        };
        let neighbor_on = Cell {
            state: true,
            rule: rule,
            activation: Activation::NeighborOn,
            omega: 0.0,
            big_omega: 1.0,
            alpha: 0.0,
        };
        let self_on = Cell {
            state: true,
            rule: rule,
            activation: Activation::SelfOn,
            omega: 0.0,
            big_omega: 1.0,
            alpha: 0.0,
        };
        let both_off = Cell {
            state: true,
            rule: rule,
            activation: Activation::BothOff,
            omega: 0.0,
            big_omega: 1.0,
            alpha: 0.0,
        };

        assert!(ColorScheme::ActivationColor.get_color(&new_state_off) == WHITE);
        assert!(ColorScheme::ActivationColor.get_color(&both_on) == BLACK);
        assert!(ColorScheme::ActivationColor.get_color(&neighbor_on) == GREEN);
        assert!(ColorScheme::ActivationColor.get_color(&self_on) == RED);
        assert!(ColorScheme::ActivationColor.get_color(&both_off) == BLUE);
    }
}
