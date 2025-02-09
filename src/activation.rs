#[derive(Clone, Copy, PartialEq)]
pub enum Activation {
    NewStateOff,
    BothOn,
    NeighborOn,
    SelfOn,
    BothOff,
}

impl Activation {
    pub fn from_state(cell_state: bool, neighbor_state: bool, new_state: bool) -> Self {
        if !new_state {
            Activation::NewStateOff // New state off
        } else if cell_state && neighbor_state {
            Activation::BothOn // Both on
        } else if !cell_state && neighbor_state {
            Activation::NeighborOn // Neighbor on
        } else if cell_state && !neighbor_state {
            Activation::SelfOn // Self on
        } else {
            Activation::BothOff // Both off
        }
    }
}
