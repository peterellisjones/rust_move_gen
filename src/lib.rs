//! # chess_move_gen
//! Provides structs and methods for generating chess moves efficiently
//!
//! Example usage:
//!
//! ```
//! use chess_move_gen::*;
//! let mut list = MoveVec::new();
//! let position = &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1").unwrap();
//! legal_moves::<MoveVec>(position, &mut list);
//! assert_eq!(list.len(), 20);
//! ```

#![feature(test)]
#![feature(platform_intrinsics)]
#![feature(const_fn)]
#![feature(binary_heap_into_iter_sorted)]

pub mod bb;
mod board;
mod cache;
mod castle;
mod castling_rights;
mod gen;
mod hash;
mod integrity;
mod mv;
mod mv_list;
mod perft;
mod piece;
mod position;
mod side;
mod square;
mod util;

#[cfg(target_feature = "sse3")]
mod dbb;

extern crate num_cpus;
extern crate rand;
extern crate threadpool;

#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate unindent;

pub use bb::BB;
pub use board::Board;
pub use castle::{Castle, KING_SIDE, QUEEN_SIDE};
pub use castling_rights::{CastlingRights, BLACK_KS, BLACK_QS, WHITE_KS, WHITE_QS};
pub use gen::legal_moves;
pub use mv::{Move, NULL_MOVE};
pub use mv_list::{MoveCounter, MoveList, MoveScore, MoveVec, PieceSquareTable, ScoredMoveList};
pub use perft::perft;
pub use perft::perft_detailed;
pub use piece::*;
pub use position::{Position, State, STARTING_POSITION_FEN};
pub use side::{Side, BLACK, WHITE};
pub use square::*;

#[cfg(target_feature = "sse3")]
pub use dbb::*;

#[test]
fn basic_functionality() {
    let mut counter = MoveCounter::new();
    let position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    let in_check = legal_moves(&position, &mut counter);
    assert!(!in_check);

    assert_eq!(counter.moves, 20);
    assert_eq!(counter.captures, 0);
    assert_eq!(counter.castles, 0);
    assert_eq!(counter.ep_captures, 0);
    assert_eq!(counter.promotions, 0);
}
