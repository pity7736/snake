use super::{constans, direction::Direction, position::Position};


pub struct Board {
    cells: Vec<Vec<String>>,
    snake_position: Position
}

impl Board {
    pub fn new() -> Self {
        let mut columns:Vec<String> = Vec::with_capacity(constans::BOARD_WIDTH);
        let mut cells: Vec<Vec<String>> = Vec::with_capacity(constans::BOARD_HEIGHT);
        for _i in 0..constans::BOARD_WIDTH {
            columns.push(".".to_string());
        }
        for _i in 0..constans::BOARD_HEIGHT {
            cells.push(columns.clone());
        }
        cells[0][0] = "$".to_string();
        Self { cells, snake_position: Position::new(0, 0) }
    }

    pub fn get_snake_position(&self) -> Position {
        return self.snake_position;
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let position = self.snake_position.move_(direction);
        self.cells[self.snake_position.row_as_usize()][self.snake_position.column_as_usize()] = String::from(".");
        self.cells[position.row_as_usize()][position.column_as_usize()] = String::from("$");
        self.snake_position = position;
    }
    
    pub fn cells(&self) -> &Vec<Vec<String>> {
        return &self.cells
    }
}
