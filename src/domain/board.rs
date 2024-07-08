use rand::Rng;

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
        let mut board = Self {
            cells,
            snake_position,
            empty_value_character,
            snake_head_character: String::from("$"),
            width,
            height
        };
        board.init();
        return board;
    }

    fn init(&mut self) {
        self.set_value_in_cells(self.snake_position, self.snake_head_character.clone());
        self.create_cookie();
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

    fn create_cookie(&mut self) {
        let position = Position::new(
            rand::thread_rng().gen_range(0..self.width).try_into().unwrap(),
            rand::thread_rng().gen_range(0..self.height).try_into().unwrap()
        );
        if self.is_cell_empty(position) {
            self.set_value_in_cells(position, String::from("#"));
        } else {
            self.create_cookie();
        }
    }

    fn is_cell_empty(&self, position: Position) -> bool {
        let cell_value = self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()].clone();
        return cell_value == self.empty_value_character;
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
