use super::{constans, direction::Direction, position::Position};


pub struct Board {
    cells: Vec<Vec<String>>,
    snake_position: Position,
    character: String
}

impl Board {
    pub fn new() -> Self {
        let mut columns:Vec<String> = Vec::with_capacity(constans::BOARD_WIDTH);
        let mut cells: Vec<Vec<String>> = Vec::with_capacity(constans::BOARD_HEIGHT);
        let character = String::from("");
        for _i in 0..constans::BOARD_WIDTH {
            columns.push(character.clone());
        }
        for _i in 0..constans::BOARD_HEIGHT {
            cells.push(columns.clone());
        }
        let snake_position = Position::new(
            constans::BOARD_WIDTH / 2,
            constans::BOARD_HEIGHT / 2
        );
        cells[snake_position.row()][snake_position.column()] = "$".to_string();
        Self { 
            cells,
            snake_position,
            character
        }
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let position = self.snake_position.move_(direction);
        if position.column() < constans::BOARD_WIDTH && position.row() < constans::BOARD_HEIGHT{
            self.cells[self.snake_position.row()][self.snake_position.column()] = self.character.clone();
            self.cells[position.row()][position.column()] = String::from("$");
            self.snake_position = position;
        }
    }

    pub fn is_snake_in_limit_position(&self) -> bool {
        return self.snake_position.column() == constans::BOARD_WIDTH - 1 || self.snake_position.row() == constans::BOARD_HEIGHT -1;
    }

    pub fn cells(&self) -> &Vec<Vec<String>> {
        return &self.cells
    }

    pub fn get_snake_position(&self) -> Position {
        return self.snake_position;
    }
}
