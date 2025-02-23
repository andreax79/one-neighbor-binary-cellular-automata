#[derive(Clone, Copy, PartialEq)]
pub enum Activation {
    NewStateOff,
    AllOff,      // 000
    RightOn,     // 001
    SelfOn,      // 010
    SelfRightOn, // 011
    LeftOn,      // 100
    LeftRightOn, // 101
    SelfLeftOn,  // 110
    AllOn,       // 111
}

impl Activation {
    pub fn from_state(
        left_neighbor_state: bool,  // left cell state
        cell_state: bool,           // cell state
        right_neighbor_state: bool, // right cell state
        new_state: bool,            // new cell state
    ) -> Self {
        if !new_state {
            Activation::NewStateOff // New state off
        } else if cell_state {
            if left_neighbor_state && right_neighbor_state {
                Activation::AllOn // Both on
            } else if left_neighbor_state {
                Activation::LeftOn // Left on
            } else if right_neighbor_state {
                Activation::RightOn // Right on
            } else {
                Activation::SelfOn // Self on
            }
        } else {
            if left_neighbor_state && right_neighbor_state {
                Activation::AllOff // Both off
            } else if left_neighbor_state {
                Activation::LeftRightOn // Left right on
            } else if right_neighbor_state {
                Activation::SelfRightOn // Self right on
            } else {
                Activation::SelfLeftOn // Self left on
            }
        }
    }
}
