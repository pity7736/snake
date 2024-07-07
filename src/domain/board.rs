use super::{direction::Direction, position::Position};


pub struct Board {
    cells: Vec<Vec<String>>,
    snake_position: Position,
    empty_value_character: String,
    snake_head_character: String,
    width: u8,
    height: u8
}

impl Board {
    pub fn new() -> Self {
        let width: u8 = 30;
        let height: u8 = 30;
        let mut columns:Vec<String> = Vec::with_capacity(width.into());
        let mut cells: Vec<Vec<String>> = Vec::with_capacity(height.into());
        let empty_value_character = String::from("");
        let snake_head_character = String::from("$");
        for _ in 0..width {
            columns.push(empty_value_character.clone());
        }
        for _ in 0..height {
            cells.push(columns.clone());
        }
        let snake_position = Position::new(
            (width / 2).try_into().unwrap(),
            (height / 2).try_into().unwrap()
        );
        cells[usize::try_from(snake_position.row()).unwrap()][usize::try_from(snake_position.column()).unwrap()] = snake_head_character.clone();
        Self {
            cells,
            snake_position,
            empty_value_character,
            snake_head_character,
            width,
            height
        }
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let current_snake_position = self.snake_position;
        let position = self.snake_position.move_(direction);
        self.snake_position = position;
        if self.is_position_within_board(position){
            self.set_value_in_cells(current_snake_position, self.empty_value_character.clone());
            self.set_value_in_cells(position, self.snake_head_character.clone());
        }
    }

    fn set_value_in_cells(&mut self, position: Position, value: String) {
        self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()] = value;
    }

    fn is_position_within_board(&self, position: Position) -> bool {
        return position.column() >= 0 &&
            position.row() >= 0 &&
            position.column() < self.width.try_into().unwrap() &&
            position.row() < self.height.try_into().unwrap();
    }

    pub fn snake_has_crashed(&self) -> bool {
        return self.snake_position.column().is_negative() ||
            self.snake_position.row().is_negative() ||
            self.snake_position.column() > self.width.try_into().unwrap() ||
            self.snake_position.row() > self.height.try_into().unwrap();
    }

    pub fn cells(&self) -> &Vec<Vec<String>> {
        return &self.cells
    }

    pub fn get_width(&self) -> u8 {
        return self.width;
    }

    pub fn get_height(&self) -> u8 {
        return self.height;
    }
}
