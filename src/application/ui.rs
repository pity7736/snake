use std::sync::mpsc::Receiver;

use crate::domain::{board::Board, direction::Direction};

pub trait UI {
    fn show(&self, board: &Board);

    fn start(&self) -> Receiver<Direction>;

    fn ask_direction(&self) -> Direction;

    fn show_lost_message(&self);

}
