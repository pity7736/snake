use rand::Rng;

use super::{constants::{COOKIE_CHARACTER, EMPTY_VALUE_CHARACTER, SNAKE_BODY_CHARACTER, SNAKE_HEAD_CHARACTER}, direction::Direction, position::Position};


pub struct Board {
    cells: Vec<Vec<char>>,
    snake: Vec<Position>,
    width: u8,
    height: u8
}

impl Board {
    pub fn new() -> Self {
        let width: u8 = 30;
        let height: u8 = 30;
        let mut columns:Vec<char> = Vec::with_capacity(width.into());
        let mut cells: Vec<Vec<char>> = Vec::with_capacity(height.into());
        for _ in 0..width {
            columns.push(EMPTY_VALUE_CHARACTER.to_owned());
        }
        for _ in 0..height {
            cells.push(columns.clone());
        }
        let mut snake: Vec<Position> = Vec::with_capacity(15);
        snake.push(Position::new(
            (width / 2).try_into().unwrap(),
            (height / 2).try_into().unwrap()
        ));
        let mut board = Self {
            cells,
            snake,
            width,
            height
        };
        board.init();
        return board;
    }

    fn init(&mut self) {
        self.set_value_in_cells(self.snake[0], SNAKE_HEAD_CHARACTER);
        self.create_cookie();
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let new_snake_position = self.snake[0].move_(direction);
        let tail_snake_position = self.snake[self.snake.len() - 1];
        let snake = self.snake.clone();
        self.snake[0] = new_snake_position;
        if self.is_position_within_board(new_snake_position) {
            for i in 1..self.snake.len() {
                self.snake[i] = snake[i - 1];
            }
            if self.is_cookie_in_position(new_snake_position) {
                self.snake.push(tail_snake_position.clone());
                self.create_cookie();
            } else {
                self.set_value_in_cells(tail_snake_position, EMPTY_VALUE_CHARACTER);
            }
            self.set_snake();
        }
    }

    fn is_position_within_board(&self, position: Position) -> bool {
        return position.column() >= 0 &&
            position.row() >= 0 &&
            position.column() < self.width.try_into().unwrap() &&
            position.row() < self.height.try_into().unwrap();
    }

    fn is_cookie_in_position(&self, position: Position) -> bool {
        let cell_value = self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()].clone();
        return cell_value == COOKIE_CHARACTER;
    }

    fn create_cookie(&mut self) {
        let position = Position::new(
            rand::thread_rng().gen_range(0..self.width).try_into().unwrap(),
            rand::thread_rng().gen_range(0..self.height).try_into().unwrap()
        );
        if self.is_cell_empty(position) {
            self.set_value_in_cells(position, COOKIE_CHARACTER);
        } else {
            self.create_cookie();
        }
    }

    fn is_cell_empty(&self, position: Position) -> bool {
        let cell_value = self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()].clone();
        return cell_value == EMPTY_VALUE_CHARACTER;
    }

    fn set_snake(&mut self) {
        self.set_value_in_cells(self.snake[0], SNAKE_HEAD_CHARACTER);
        for i in 1..self.snake.len() {
            self.set_value_in_cells(self.snake[i], SNAKE_BODY_CHARACTER);
        }
    }

    fn set_value_in_cells(&mut self, position: Position, value: char) {
        self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()] = value;
    }

    pub fn snake_has_crashed(&self) -> bool {
        return self.snake[0].column().is_negative() ||
            self.snake[0].row().is_negative() ||
            self.snake[0].column() > self.width.try_into().unwrap() ||
            self.snake[0].row() > self.height.try_into().unwrap();
    }

    pub fn cells(&self) -> &Vec<Vec<char>> {
        return &self.cells
    }

    pub fn get_width(&self) -> u8 {
        return self.width;
    }

    pub fn get_height(&self) -> u8 {
        return self.height;
    }
}
