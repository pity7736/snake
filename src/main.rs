use snake::{application::play_controller::PlayController, presentation::console::ui::ConsoleUI};




fn main() {
    let ui = ConsoleUI::new();
    let mut play_controller = PlayController::new(&ui);
    play_controller.play()
} 
