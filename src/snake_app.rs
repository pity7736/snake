use crate::{application::{play_controller::PlayController, ui::UI}, presentation::console::ui::ConsoleUI};


pub struct SnakeApp{
}

impl SnakeApp {
    pub fn new() -> Self {
        Self { }
    }

    pub fn start(&self) {
        let mut ui = ConsoleUI::new(PlayController::new());
        ui.start()
    }
}
