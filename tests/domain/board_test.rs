use snake::domain::board::Board;
use snake::domain::constants::COOKIE_CHARACTER;
use snake::domain::direction::Direction;
use snake::domain::position::Position;


#[test]
pub fn inital_snake_position() {
    let board = Board::new();

    let cells = board.cells();

    assert_eq!("$", cells[get_usize_from_u8(board.get_width() / 2)][get_usize_from_u8(board.get_height() / 2)])
}

#[test]
pub fn move_snake_right() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(
        (board.get_width() / 2).try_into().unwrap(),
        (board.get_height() / 2).try_into().unwrap()
    );

    board.move_snake(Direction::RIGHT);

    let snake_position = Position::new(initial_snake_position.row(), initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[get_usize_from_i8(snake_position.row())][get_usize_from_i8(snake_position.column())]);
    assert_eq!("", cells[get_usize_from_i8(initial_snake_position.row())][get_usize_from_i8(initial_snake_position.column())]);
    assert_eq!(false, board.snake_has_crashed());
}

#[test]
pub fn move_snake_down() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(
        (board.get_width() / 2).try_into().unwrap(),
        (board.get_height() / 2).try_into().unwrap()
    );

    board.move_snake(Direction::DOWN);

    let cells = board.cells();
    let snake_position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column());

    assert_eq!("$", cells[get_usize_from_i8(snake_position.row())][get_usize_from_i8(snake_position.column())]);
    assert_eq!("", cells[get_usize_from_i8(initial_snake_position.row())][get_usize_from_i8(initial_snake_position.column())]);
    assert_eq!(false, board.snake_has_crashed());
}

#[test]
pub fn move_snake_up() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(
        (board.get_width() / 2).try_into().unwrap(),
        (board.get_height() / 2).try_into().unwrap()
    );

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::UP);

    let snake_position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[get_usize_from_i8(snake_position.row())][get_usize_from_i8(snake_position.column())]);
    assert_eq!("", cells[get_usize_from_i8(initial_snake_position.row())][get_usize_from_i8(initial_snake_position.column())]);
}

#[test]
pub fn move_snake_left() {
    let mut board = Board::new();
    let initial_snake_position = Position::new(
        (board.get_width() / 2).try_into().unwrap(),
        (board.get_height() / 2).try_into().unwrap()
    );

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::LEFT);

    let snake_position = Position::new(initial_snake_position.row() + 1, initial_snake_position.column() + 1);
    let cells = board.cells();

    assert_eq!("$", cells[get_usize_from_i8(snake_position.row())][get_usize_from_i8(snake_position.column())]);
    assert_eq!("", cells[get_usize_from_i8(initial_snake_position.row())][get_usize_from_i8(initial_snake_position.column())]);
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

#[test]
fn snake_left_crash() {
    let mut board = Board::new();
    for _ in 0..(board.get_width() / 2 + 1){
        board.move_snake(Direction::LEFT)
    }

    assert_eq!(true, board.snake_has_crashed());
}

#[test]
fn snake_up_crash() {
    let mut board = Board::new();

    for _ in 0..board.get_height() / 2 + 1 {
        board.move_snake(Direction::UP)
    }

    assert_eq!(true, board.snake_has_crashed());
}

#[test]
fn cookie_is_created() {
    let mut count: i8 = 0;
    let board = Board::new();
    for row in board.cells() {
        for cell in row {
           if *cell == COOKIE_CHARACTER {
               count += 1
           }
        }
    }

    assert_eq!(1, count);
}


fn get_usize_from_u8(value: u8) -> usize {
    return usize::try_from(value).unwrap();
}

fn get_usize_from_i8(value: i8) -> usize {
    return usize::try_from(value).unwrap();
}
