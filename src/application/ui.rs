use crate::domain::board::Board;

pub trait UI {
    fn show(&self, board: &Board);

    fn start(&mut self);
}
