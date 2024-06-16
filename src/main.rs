use snake::{
    presentation::console::board_shower::ConsoleBoardShower,
    snake_app::SnakeApp
};




fn main() {
    let shower = ConsoleBoardShower::new();
    let mut app = SnakeApp::new(&shower);
    app.start()
} 
