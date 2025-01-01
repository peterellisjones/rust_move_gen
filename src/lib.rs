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
//!
//! Ways to store moves:
//!
//! MoveVec: Wraps a vector of generated moves, useful if you need to access the actual moves being generated
//! MoveCounter: Counts moves of each kind (captures, castles, promotions etc). Useful if you are making a perft function or need statistics about moves for a position, but don't care about the actual moves
//! SortedMoveAdder + SortedMoveHeap: Stores genarated moves in a sorted binary heap, which are efficiently ordered as they are inserted based on a heuristic scoring and piece-square table that you provide. Use this if you want the moves to have a reasonably good initial ordering so moves that are checked first are more likely to lead to eg alpha-beta cutoffs and reduce the search tree size.

#![feature(test)]
#![feature(portable_simd)]
#![feature(binary_heap_into_iter_sorted)]

pub mod bb;
mod board;
mod cache;
mod castle;
mod castling_rights;
mod generation;
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

pub use crate::bb::BB;
pub use crate::castle::{Castle, KING_SIDE, QUEEN_SIDE};
pub use crate::castling_rights::{BLACK_KS, BLACK_QS, CastlingRights, WHITE_KS, WHITE_QS};
pub use crate::generation::{
    MoveGenPreprocessing, legal_moves, legal_moves_with_preprocessing, loud_legal_moves,
    loud_legal_moves_with_preprocessing, movegen_preprocessing,
};
pub use crate::mv::{KING_SIDE_CASTLE, Move, MoveScore, NULL_MOVE, QUEEN_SIDE_CASTLE};
pub use crate::mv_list::{
    MoveAdder, MoveCounter, MoveVec, PieceSquareTable, SortedMoveAdder, SortedMoveHeap,
    SortedMoveHeapItem,
};
pub use crate::piece::*;
pub use crate::position::{Position, STARTING_POSITION_FEN, State};
pub use crate::side::{BLACK, Side, WHITE};
pub use crate::square::*;
pub use board::Board;
pub use perft::perft;
pub use perft::perft_detailed;

#[cfg(target_feature = "sse3")]
pub use crate::dbb::*;

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
