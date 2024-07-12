use super::direction::Direction;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    row: i8,
    column: i8
}

impl Position {

    pub fn new(row: i8, column: i8) -> Self {
        Self { row, column }
    }

    pub fn new_from_usize(row: usize, column: usize) -> Self {
        Self {
            row: row.try_into().unwrap(),
            column: column.try_into().unwrap()
        }
    }

    pub fn move_(&self, direction: Direction) -> Self {
        match direction {
            Direction::DOWN => return Self {row: self.row + 1, column: self.column},
            Direction::RIGHT => return Self {row: self.row, column: self.column + 1},
            Direction::UP => return Self {row: self.row - 1, column: self.column},
            Direction::LEFT => return Self {row: self.row, column: self.column - 1},
        }
    }

    pub fn row(&self) -> i8 {
        return self.row
    }

    pub fn column(&self) -> i8 {
        return self.column
    }
}
