use super::direction::Direction;

#[derive(Copy, Clone)]
pub struct Position {
    row: u8,
    column: u8
}

impl Position {

    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }

    pub fn move_(&self, direction: Direction) -> Self {
        match direction {
            Direction::DOWN => return Self {row: self.row + 1, column: self.column},
            Direction::RIGHT => return Self {row: self.row, column: self.column + 1},
            Direction::UP => todo!(),
            Direction::LEFT => todo!(),
        }
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn row_as_usize(&self) -> usize {
        return usize::from(self.row);
    }
    
    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn column_as_usize(&self) -> usize {
        return usize::from(self.column);
    }
}
