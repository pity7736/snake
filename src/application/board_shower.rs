use crate::domain::board::Board;


pub trait BoardShower {
    fn show(&self, board: &Board);
}
