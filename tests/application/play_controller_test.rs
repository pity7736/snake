use std::sync::mpsc::{channel, Receiver};

use mockall::mock;
use snake::{
    application::{play_controller::PlayController, ui::UI},
    domain::{board::Board, direction::Direction}
};


#[test]
fn play() {
    mock!{
        pub MyUI {}

        impl UI for MyUI {

            fn show(&self, board: &Board);

            fn start(&self) -> Receiver<Direction>;
        
            fn ask_direction(&self) -> Direction;
        }
    }
    let mut ui_mock = MockMyUI::new();
    ui_mock.expect_show().times(2..).returning(|_| ());
    ui_mock.expect_ask_direction().returning(|| Direction::RIGHT);
    let (_sender, receiver) = channel::<Direction>();
    ui_mock.expect_start().times(1).return_once(|| receiver);
    let mut play_controller = PlayController::new(&ui_mock);

    play_controller.play();

    ui_mock.checkpoint();
}
