use std::{io, sync::mpsc::{self, Receiver}, thread::{self, sleep}, time::Duration};

use crate::{
    application::ui::UI,
    domain::{
        board::Board,
        constans,
        direction::Direction
    }
};


pub struct ConsoleUI{}

impl ConsoleUI {

    pub fn new() -> Self {
        Self {  }
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

    fn start(&self) -> Receiver<Direction> {
        let (sender, receiver) = mpsc::channel::<Direction>();
        thread::spawn(move || {
            loop {
                let direction_result = ask_direction();
                if direction_result.is_some() {
                    let _ = sender.send(direction_result.unwrap());
                }
            }
        });
        return receiver;
    }

    fn ask_direction(&self) -> Direction {
        let direction_result = ask_direction();
        if direction_result.is_some() {
            return direction_result.unwrap();
        }
        return self.ask_direction();
    }

    fn show_lost_message(&self) {
        println!("¡Perdiste!")
    }
}


fn ask_direction() -> Option<Direction>{
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
