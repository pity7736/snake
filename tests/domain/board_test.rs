use snake::domain::board::Board;
use snake::domain::constants::{COOKIE_CHARACTER, EMPTY_VALUE_CHARACTER, SNAKE_BODY_CHARACTER, SNAKE_HEAD_CHARACTER};
use snake::domain::direction::Direction;
use snake::domain::position::Position;


#[test]
pub fn inital_snake_position() {
    let board = Board::new();

    let cells = board.cells();
    let snake_position = Position::new(
        ((board.get_width() / 2)).try_into().unwrap(),
        (board.get_height() / 2).to_owned().try_into().unwrap()
    );

    assert!(is_snake_head_in_position(cells, snake_position));
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

    assert!(is_snake_head_in_position(cells, snake_position));
    assert!(is_cell_empty(cells, initial_snake_position));
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

    assert!(is_snake_head_in_position(cells, snake_position));
    assert!(is_cell_empty(cells, initial_snake_position));
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

    assert!(is_snake_head_in_position(cells, snake_position));
    assert!(is_cell_empty(cells, initial_snake_position));
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

    assert!(is_snake_head_in_position(cells, snake_position));
    assert!(is_cell_empty(cells, initial_snake_position));
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

#[test]
fn eat_cookie() {
    let mut board = Board::new();
    let cookie_position = get_cookie_position(&board).unwrap();
    let snake_position = Position::new(
        ((board.get_width() / 2)).try_into().unwrap(),
        (board.get_height() / 2).to_owned().try_into().unwrap()
    );
    let oposite_snake_direction: Direction;
    (_, oposite_snake_direction) = _eat_cookie(&mut board, &cookie_position, snake_position);
    let old_snake_position = cookie_position.move_(oposite_snake_direction);

    assert!(is_snake_head_in_position(board.cells(), cookie_position));
    assert!(is_snake_body_in_position(board.cells(), old_snake_position));
    assert!(check_one_snake_head(board.cells()));
}

#[test]
fn eat_cookie_and_move() {
    let mut board = Board::new();
    let cookie_position = get_cookie_position(&board).unwrap();
    let snake_position = Position::new(
        ((board.get_width() / 2)).try_into().unwrap(),
        (board.get_height() / 2).to_owned().try_into().unwrap()
    );
    let oposite_snake_direction: Direction;
    let current_snake_direction: Direction;

    (current_snake_direction, oposite_snake_direction) = _eat_cookie(&mut board, &cookie_position, snake_position);
    let old_snake_position = cookie_position.move_(oposite_snake_direction);
    board.move_snake(current_snake_direction);

    assert!(is_snake_head_in_position(board.cells(), cookie_position.move_(current_snake_direction)));
    assert!(is_snake_body_in_position(board.cells(), cookie_position));
    assert!(is_cell_empty(board.cells(), old_snake_position));
}

#[test]
fn eat_cookie_twice_and_move() {
    let mut board = Board::new();
    let mut cookie_position = get_cookie_position(&board).unwrap();
    let mut snake_position = get_snake_position(&board).unwrap();
    let oposite_snake_direction: Direction;
    let current_snake_direction: Direction;

    _eat_cookie(&mut board, &cookie_position, snake_position);
    cookie_position = get_cookie_position(&board).unwrap();
    snake_position = get_snake_position(&board).unwrap();
    (current_snake_direction, oposite_snake_direction) = _eat_cookie(&mut board, &cookie_position, snake_position);
    let old_snake_position = cookie_position.move_(oposite_snake_direction).move_(oposite_snake_direction);
    board.move_snake(current_snake_direction);

    assert!(is_snake_head_in_position(board.cells(), cookie_position.move_(current_snake_direction)));
    assert!(is_snake_body_in_position(board.cells(), cookie_position));
    assert!(is_cell_empty(board.cells(), old_snake_position));
}

fn get_cookie_position(board: &Board) -> Option<Position> {
    for (row_index, row) in board.cells().iter().enumerate() {
        for (cell_index,cell) in row.iter().enumerate() {
           if *cell == COOKIE_CHARACTER {
               return Some(Position::new(row_index.try_into().unwrap(), cell_index.try_into().unwrap()));
           }
        }
    }
    return None;
}

fn get_snake_position(board: &Board) -> Option<Position> {
    for (row_index, row) in board.cells().iter().enumerate() {
        for (cell_index,cell) in row.iter().enumerate() {
           if *cell == SNAKE_HEAD_CHARACTER {
               return Some(Position::new(row_index.try_into().unwrap(), cell_index.try_into().unwrap()));
           }
        }
    }
    return None;
}

fn _eat_cookie(board: &mut Board, cookie_position: &Position, mut snake_position: Position) -> (Direction, Direction) {
    let mut oposite_snake_direction = Direction::DOWN;
    let mut current_snake_direction = Direction::RIGHT;
    while cookie_position.row() > snake_position.row() {
        snake_position = snake_position.move_(Direction::DOWN);
        board.move_snake(Direction::DOWN);
        oposite_snake_direction = Direction::UP;
        current_snake_direction = Direction::DOWN;
    }
    while cookie_position.row() < snake_position.row() {
        snake_position = snake_position.move_(Direction::UP);
        board.move_snake(Direction::UP);
        oposite_snake_direction = Direction::DOWN;
        current_snake_direction = Direction::UP;
    }
    while cookie_position.column() > snake_position.column() {
        snake_position = snake_position.move_(Direction::RIGHT);
        board.move_snake(Direction::RIGHT);
        oposite_snake_direction = Direction::LEFT;
        current_snake_direction = Direction::RIGHT;
    }
    while cookie_position.column() < snake_position.column() {
        snake_position = snake_position.move_(Direction::LEFT);
        board.move_snake(Direction::LEFT);
        oposite_snake_direction = Direction::RIGHT;
        current_snake_direction = Direction::LEFT;
    }
    return (current_snake_direction, oposite_snake_direction);
}

fn check_one_snake_head(cells: &Vec<Vec<char>>) -> bool {
    let mut count = 0;
    for row in cells {
        for cell in row {
           if *cell == SNAKE_HEAD_CHARACTER {
               count += 1;
           }
        }
    }
    return count == 1;
}

fn is_snake_head_in_position(cells: &Vec<Vec<char>>, position: Position) -> bool {
    return SNAKE_HEAD_CHARACTER == cells[get_usize_from_i8(position.row())][get_usize_from_i8(position.column())];
}

fn is_snake_body_in_position(cells: &Vec<Vec<char>>, position: Position) -> bool {
    return SNAKE_BODY_CHARACTER == cells[get_usize_from_i8(position.row())][get_usize_from_i8(position.column())];
}

fn is_cell_empty(cells: &Vec<Vec<char>>, position: Position) -> bool {
    return EMPTY_VALUE_CHARACTER == cells[get_usize_from_i8(position.row())][get_usize_from_i8(position.column())];
}

fn get_usize_from_i8(value: i8) -> usize {
    return usize::try_from(value).unwrap();
}
