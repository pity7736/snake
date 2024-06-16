use super::direction::Direction;

#[derive(Copy, Clone)]
pub struct Position {
    row: usize,
    column: usize
}

impl Position {

    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn move_(&self, direction: Direction) -> Self {
        match direction {
            Direction::DOWN => return Self {row: self.row + 1, column: self.column},
            Direction::RIGHT => return Self {row: self.row, column: self.column + 1},
            Direction::UP => return Self {row: self.row - 1, column: self.column},
            Direction::LEFT => return Self {row: self.row, column: self.column - 1},
        }
    }

    pub fn row(&self) -> usize {
        return self.row
    }
    
    pub fn column(&self) -> usize {
        return self.column
    }
}
