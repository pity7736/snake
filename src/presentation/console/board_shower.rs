use crate::{application::board_shower::BoardShower, domain::{board::Board, constans}};


pub struct ConsoleBoardShower{
}

impl ConsoleBoardShower {
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

impl BoardShower for ConsoleBoardShower{
    fn show(&self, board: &Board) {
        print!("{esc}c", esc = 27 as char);
        ConsoleBoardShower::print_horizontal_line();
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
        ConsoleBoardShower::print_horizontal_line();
        println!("");
    }
}
