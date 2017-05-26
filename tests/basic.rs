extern crate chess_move_gen;

use chess_move_gen::*;

#[test]
fn basic_functionality() {
    let mut counter = MoveCounter::new();
    let board = Board::from_fen(STARTING_POSITION_FEN).unwrap();

    let in_check = legal_moves(&board, &mut counter);
    assert!(!in_check);

    assert_eq!(counter.moves, 20);
    assert_eq!(counter.captures, 0);
    assert_eq!(counter.castles, 0);
    assert_eq!(counter.ep_captures, 0);
    assert_eq!(counter.promotions, 0);
}