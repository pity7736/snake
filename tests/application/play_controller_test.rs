// use mockall::mock;
// use snake::{
//     application::{play_controller::PlayController, ui::UI},
//     domain::{board::Board, direction::Direction}
// };


// #[test]
// fn play() {
//     mock!{
//         pub MyUI {}

//         impl UI for MyUI {

//             fn show(&self, board: &Board) {}

//             fn ask_direction(&self) -> Option<Direction> {}
//         }
//     }
//     let mut ui_mock = MockMyUI::new();
//     ui_mock.expect_show().times(2..).returning(move |_| ());
//     let initial_direction = Direction::RIGHT;
//     ui_mock.expect_ask_direction().times(1..).returning(move | | Some(initial_direction));
//     let mut play_controller = PlayController::new(&ui_mock);

//     play_controller.play();

//     ui_mock.checkpoint();
// }
