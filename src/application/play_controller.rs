use crate::domain::board::Board;

use super::ui::UI;

pub struct PlayController<'a> {
    board: Board,
    ui: &'a dyn UI
}

impl<'a> PlayController<'a> {

    pub fn new(ui: &'a dyn UI) -> Self {
        Self { board: Board::new(), ui }
    }

    pub fn play(&mut self) {
        self.ui.show(&self.board);
        let mut direction = self.ui.ask_direction();
        let receiver = self.ui.start();
        loop {
            if self.is_playing() {
                self.board.move_snake(direction);
                self.ui.show(&self.board);
                let direction_result = receiver.try_recv();
                if direction_result.is_ok() {
                    direction = direction_result.unwrap();
                }
            } else {
                break;
            }
        }
    }

    fn is_playing(&self) -> bool {
        return !self.board.is_snake_in_limit_position();
    }

}