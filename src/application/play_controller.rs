use crate::domain::{board::Board, direction::Direction};

pub struct PlayController {
    board: Board,
    direction: Option<Direction>
}

impl PlayController {

    pub fn new() -> Self {
        Self {  board: Board::new(), direction: None}
    }

    pub fn play(&mut self) {
        if self.board.is_snake_in_limit_position() {
            return;
        } else {
            self.board.move_snake(self.direction.unwrap());
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }
    
    pub fn board(&self) -> &Board {
        &self.board
    }
}
