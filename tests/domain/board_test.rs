use snake::domain::board::Board;
use snake::domain::direction::Direction;


#[test]
pub fn inital_snake_position() {
    let board = Board::new();

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(0, position.row());
    assert_eq!(0, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_right() {
    let mut board = Board::new();

    board.move_snake(Direction::RIGHT);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(0, position.row());
    assert_eq!(1, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_down() {
    let mut board = Board::new();

    board.move_snake(Direction::DOWN);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(1, position.row());
    assert_eq!(0, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_up() {
    let mut board = Board::new();

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::UP);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(1, position.row());
    assert_eq!(1, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}

#[test]
pub fn move_snake_left() {
    let mut board = Board::new();

    board.move_snake(Direction::DOWN);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::RIGHT);
    board.move_snake(Direction::LEFT);

    let position = board.get_snake_position();
    let cells = board.cells();

    assert_eq!(1, position.row());
    assert_eq!(1, position.column()) ;
    assert_eq!("$", cells[position.row()][position.column()])
}
