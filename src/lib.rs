//! # chess_move_gen
//! Provides structs and methods for generating chess moves efficiently
//!
//! Example usage:
//!
//! ```
//! use chess_move_gen::*;
//! let mut list = MoveVec::new();
//! let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1").unwrap();
//! legal_moves::<MoveVec>(board, &mut list);
//! assert_eq!(list.len(), 20);
//! ```

#![feature(test)]
#![feature(cfg_target_feature)]
#![feature(platform_intrinsics)]
#![feature(const_fn)]

pub mod bb;
mod board;
mod castle;
mod castling_rights;
mod gen;
mod integrity;
mod mv;
mod mv_list;
mod piece;
mod side;
mod square;
mod util;
mod hash;
mod tree;

#[cfg(target_feature = "sse3")]
mod dbb;

extern crate rand;
#[cfg(test)]
extern crate unindent;
#[cfg(test)]
extern crate test;

pub use board::{Board, State, STARTING_POSITION_FEN};
pub use tree::Tree;
pub use gen::legal_moves;
pub use castle::{Castle, KING_SIDE, QUEEN_SIDE};
pub use castling_rights::{CastlingRights, BLACK_QS, BLACK_KS, WHITE_QS, WHITE_KS};
pub use mv::Move;
pub use mv_list::{MoveList, MoveCounter, MoveVec};
pub use side::{Side, WHITE, BLACK};
pub use piece::*;
pub use square::*;
pub use bb::BB;

#[cfg(target_feature = "sse3")]
pub use dbb::*;

#[cfg(test)]
fn perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        let mut counter = MoveCounter::new();
        legal_moves(&board, &mut counter);
        return counter.moves as usize;
    }

    let mut moves = MoveVec::new();
    legal_moves(&board, &mut moves);

    let state = board.state().clone();
    let key = board.hash_key();
    let mut count = 0;
    for &mv in moves.iter() {
        let capture = board.make(mv);

        count += perft(board, depth - 1);

        board.unmake(mv, capture, &state, key);
    }

    count
}

#[test]
fn perft_test() {
    let mut board = Board::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut board, 3), 197281);
}

#[bench]
fn perft_bench_starting_position(b: &mut test::Bencher) {
    let mut board = Board::from_fen(STARTING_POSITION_FEN).unwrap();

    b.iter(|| -> usize { perft(&mut board, 2) });
}