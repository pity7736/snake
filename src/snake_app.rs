use crate::{application::play_controller::PlayController, presentation::console::ui::ConsoleUI};


pub struct SnakeApp{
}

impl SnakeApp {
    pub fn new() -> Self {
        Self { }
    }

    pub fn start(&self) {
        let ui = ConsoleUI::new();
        let mut play_controller = PlayController::new(&ui);
        play_controller.play()
    }
}
