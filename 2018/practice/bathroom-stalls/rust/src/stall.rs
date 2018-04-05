#[derive(Debug, Clone)]
pub enum Stall {
    Empty,
    Occupied,
}

pub type Stalls = Vec<Stall>;

pub fn new_stalls(n: u64) -> Stalls {
    let size = n as usize;
    let mut stalls = vec![Stall::Empty; size + 2];

    stalls[0] = Stall::Occupied;
    stalls[size + 1] = Stall::Occupied;

    stalls
}
