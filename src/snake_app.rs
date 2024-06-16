use std::{thread::sleep, time::Duration};

use crate::{
    application::board_shower::BoardShower,
    domain::{board::Board, direction::Direction}
};

pub struct SnakeApp<'a> {
    board: Board,
    shower: &'a dyn BoardShower
}

impl<'a> SnakeApp<'a> {

    pub fn new(shower: &'a dyn BoardShower) -> Self {
        Self {board: Board::new(), shower}
    }

    pub fn start(&mut self) {
        loop {
            self.shower.show(&self.board);
            self.board.move_snake(Direction::DOWN);
            sleep(Duration::from_millis(300));
            self.shower.show(&self.board)       
        }
    }
}
 