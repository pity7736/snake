use std::{io, sync::mpsc, thread::{self, sleep}, time::Duration};

use crate::{
    application::{play_controller::PlayController, ui::UI},
    domain::{
        board::Board,
        constans,
        direction::Direction
    }
};


pub struct ConsoleUI{
    play_controller: PlayController
}

impl ConsoleUI {
    pub fn new(play_controller: PlayController) -> Self {
        Self { play_controller }
    }

    fn print_horizontal_line() {
        for _ in 0..constans::BOARD_WIDTH {
            print!("--");
        }
        print!("-");
    }
}


impl UI for ConsoleUI{

    fn show(&self, board: &Board) {
        sleep(Duration::from_millis(100));
        print!("{esc}c", esc = 27 as char);
        ConsoleUI::print_horizontal_line();
        println!("");
        for row in board.cells() {
            print!("|");
            for cell in row {
                if cell.is_empty() {
                    print!(". ");
                } else {
                    print!("{} ", cell)
                }
            }
            print!("|");
            println!("");
        }
        ConsoleUI::print_horizontal_line();
        println!("");
    }

    fn start(&mut self) {
        self.show(self.play_controller.board());
        let d = _ask_direction();
        if d.is_some() {
            self.play_controller.set_direction(d.unwrap());
            let (sender, receiver) = mpsc::channel::<Direction>();
            thread::spawn(move || {
                loop {
                    let key = _ask_direction();
                    if key.is_some() {
                        let _ = sender.send(key.unwrap());
                    }
                }
            });
            loop {
                self.show(self.play_controller.board());
                let m = receiver.try_recv();
                if m.is_ok() {
                    self.play_controller.set_direction(m.unwrap())
                }
                self.play_controller.play();
            }
        }
    }
}


fn _ask_direction() -> Option<Direction>{
    let mut direction_input = String::new();
    io::stdin()
        .read_line(&mut direction_input)
        .expect("fail");
    direction_input = direction_input.trim().parse().expect("oaeuoaeu");
    if direction_input.eq("u") {
        return Some(Direction::RIGHT);
    } else if direction_input.eq("e"){
        return Some(Direction::DOWN);
    } else if direction_input.eq("a"){
        return Some(Direction::LEFT);
    } else if direction_input.eq("."){
        return Some(Direction::UP);
    } else {
        return None;
    }
}
