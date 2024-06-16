use snake::domain::board::Board;
use snake::domain::constans;
use snake::domain::direction::Direction;


#[test]
pub fn inital_snake_position() {
    let board = Board::new();

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(constans::BOARD_HEIGHT / 2, position.row());
    assert_eq!(constans::BOARD_WIDTH / 2, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_right() {
    let mut board = Board::new();
    let initial_snake_position = board.get_snake_position();

    board.move_snake(Direction::RIGHT);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(initial_snake_position.row(), position.row());
    assert_eq!(initial_snake_position.column() + 1, position.column());
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_down() {
    let mut board = Board::new();
    let initial_snake_position = board.get_snake_position();

    board.move_snake(Direction::DOWN);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(initial_snake_position.row() + 1, position.row());
    assert_eq!(initial_snake_position.column(), position.column());
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_up() {
    let mut board = Board::new();
    let initial_snake_position = board.get_snake_position();

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::UP);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(initial_snake_position.row() + 1, position.row());
    assert_eq!(initial_snake_position.column() + 1, position.column());
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_left() {
    let mut board = Board::new();
    let initial_snake_position = board.get_snake_position();

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::LEFT);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(initial_snake_position.row() + 1, position.row());
    assert_eq!(initial_snake_position.column() + 1, position.column());
    assert_eq!("$", cells[position.row()][position.column()])
}
