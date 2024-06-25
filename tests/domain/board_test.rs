use snake::domain::board::Board;
use snake::domain::direction::Direction;
use snake::domain::position::Position;


#[test]
pub fn inital_snake_position() {
    let board = Board::new();

    let cells = board.cells();

    assert_eq!("$", cells[board.get_width() / 2][board.get_height() / 2])
}

#[test]
pub fn move_snake_right() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(board.get_width() /2, board.get_height() / 2);

    board.move_snake(Direction::RIGHT);

    let snake_position = Position::new(initial_snake_position.row(), initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[snake_position.row()][snake_position.column()]);
    assert_eq!("", cells[initial_snake_position.row()][initial_snake_position.column()]);
    assert_eq!(false, board.snake_has_crashed());
}

#[test]
pub fn move_snake_down() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(board.get_width() /2, board.get_height() / 2);

    board.move_snake(Direction::DOWN);

    let cells = board.cells();
    let snake_position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column());

    assert_eq!("$", cells[snake_position.row()][snake_position.column()]);
    assert_eq!("", cells[initial_snake_position.row()][initial_snake_position.column()]);
    assert_eq!(false, board.snake_has_crashed());
}

#[test]
pub fn move_snake_up() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(board.get_width() /2, board.get_height() / 2);

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::UP);

    let snake_position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[snake_position.row()][snake_position.column()]);
}

#[test]
pub fn move_snake_left() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(board.get_width() /2, board.get_height() / 2);

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::LEFT);

    let position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[position.row()][position.column()]);
}

#[test]
fn snake_right_crash() {
    let mut board = Board::new();
    for _ in 0..(board.get_width() / 2 + 1){
        board.move_snake(Direction::RIGHT)
    }

    assert_eq!(true, board.snake_has_crashed());
}

#[test]
fn snake_down_crash() {
    let mut board = Board::new();

    for _ in 0..board.get_height() / 2 + 1 {
        board.move_snake(Direction::DOWN)
    }

    assert_eq!(true, board.snake_has_crashed());
}
