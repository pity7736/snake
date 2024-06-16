use std::{io, sync::mpsc, thread::{self, sleep}, time::Duration};

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
        self.shower.show(&self.board);
        let mut direction = SnakeApp::ask_direction();
        let mut new_direction = direction;
        let (sender, receiver) = mpsc::channel::<Direction>();
        thread::spawn(move || {
            loop {
                let _ = sender.send(SnakeApp::ask_direction());   
            }
        });
        loop {
            let m = receiver.try_recv();
            sleep(Duration::from_millis(100));
            if m.is_ok() {
                new_direction = m.unwrap();
                direction = new_direction;
            } else {
                new_direction = direction;
            }

            self.board.move_snake(new_direction);
            self.shower.show(&self.board)       
        }
    }

    fn ask_direction() -> Direction {
        let mut direction_input = String::new();
        io::stdin()
            .read_line(&mut direction_input)
            .expect("fail");
        direction_input = direction_input.trim().parse().expect("oaeuoaeu");
        if direction_input.eq("u") {
            return Direction::RIGHT;
        } else if direction_input.eq("e"){
            return Direction::DOWN;
        } else if direction_input.eq("a"){
            return Direction::LEFT;
        } else {
            return Direction::UP;
        }
    }
}
 