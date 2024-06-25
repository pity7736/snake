#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    
    pub fn is_opposite_of(self, direction: Self) -> bool {
        match self {
            Direction::UP => direction == Direction::DOWN,
            Direction::DOWN => direction == Direction::UP,
            Direction::LEFT => direction == Direction::RIGHT,
            Direction::RIGHT => direction == Direction::LEFT,
        }
    }
}
