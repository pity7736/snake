use super::{direction::Direction, position::Position};


pub struct Board {
    cells: Vec<Vec<String>>,
    snake_position: Position,
    character: String,
    width: usize,
    height: usize
}

impl Board {
    pub fn new() -> Self {
        let width = 30;
        let height = 30;
        let mut columns:Vec<String> = Vec::with_capacity(width);
        let mut cells: Vec<Vec<String>> = Vec::with_capacity(height);
        let character = String::from("");
        for _i in 0..width {
            columns.push(character.clone());
        }
        for _i in 0..height {
            cells.push(columns.clone());
        }
        let snake_position = Position::new(
            width / 2,
            height / 2
        );
        cells[snake_position.row()][snake_position.column()] = "$".to_string();
        Self { 
            cells,
            snake_position,
            character,
            width,
            height
        }
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let current_snake_position = self.snake_position;
        let position = self.snake_position.move_(direction);
        self.snake_position = position;
        if position.column() < self.width && position.row() < self.height{
            self.cells[current_snake_position.row()][current_snake_position.column()] = self.character.clone();
            self.cells[position.row()][position.column()] = String::from("$");
        }
    }

    pub fn snake_has_crashed(&self) -> bool {
        return self.snake_position.column() > self.width || self.snake_position.row() > self.height;
    }

    pub fn cells(&self) -> &Vec<Vec<String>> {
        return &self.cells
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }
}
