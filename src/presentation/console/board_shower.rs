use crate::{application::board_shower::BoardShower, domain::board::Board};


pub struct ConsoleBoardShower{
}

impl ConsoleBoardShower {
    pub fn new() -> Self {
        Self {  }
    }

}

impl BoardShower for ConsoleBoardShower{
    fn show(&self, board: &Board) {
        print!("{esc}c", esc = 27 as char);
        for _ in 0..board.cells()[0].len() {
            print!("-")
        }
        println!("");
        for row in board.cells() {
            for cell in row {
                print!("{}", cell);
            }
            println!("");
        }
        println!("");
        for _ in 0..board.cells()[0].len() {
            print!("-")
        }
        println!("");
    }
}
