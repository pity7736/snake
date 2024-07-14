use rand::Rng;

use super::{constants::{COOKIE_CHARACTER, EMPTY_VALUE_CHARACTER, SNAKE_BODY_CHARACTER, SNAKE_HEAD_CHARACTER}, direction::Direction, position::Position, snake::Snake};


pub struct Board {
    cells: Vec<Vec<char>>,
    snake: Snake,
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
        let snake = Snake::new(Position::new(
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
        self.set_value_in_cells(self.snake.head(), SNAKE_HEAD_CHARACTER);
        self.create_cookie();
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let tail_snake_position = self.snake.tail();
        self.snake.move_(direction);
        if self.is_position_within_board(self.snake.head()) {
            if self.is_cookie_in_position(self.snake.head()) {
                self.snake.eat();
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
        if !self.snake_has_crashed() {
            self.set_value_in_cells(self.snake.head(), SNAKE_HEAD_CHARACTER);
            for position in self.snake.body() {
                self.set_value_in_cells(position, SNAKE_BODY_CHARACTER);
            }
        }
    }

    fn set_value_in_cells(&mut self, position: Position, value: char) {
        self.cells[usize::try_from(position.row()).unwrap()][usize::try_from(position.column()).unwrap()] = value;
    }

    pub fn snake_has_crashed(&self) -> bool {
        return self.snake.head().column().is_negative() ||
            self.snake.head().row().is_negative() ||
            self.snake.head().column() > self.width.try_into().unwrap() ||
            self.snake.head().row() > self.height.try_into().unwrap() || 
            self.snake.has_crashed() == true;
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
