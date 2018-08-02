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

pub mod bb;
mod board;
mod castle;
mod castling_rights;
mod gen;
mod hash;
mod integrity;
mod mv;
mod mv_list;
mod piece;
mod position;
mod side;
mod square;
mod util;

#[cfg(target_feature = "sse3")]
mod dbb;

extern crate rand;
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
pub use mv_list::{MoveCounter, MoveList, MoveVec};
pub use piece::*;
pub use position::{Position, State, STARTING_POSITION_FEN};
pub use side::{Side, BLACK, WHITE};
pub use square::*;

#[cfg(target_feature = "sse3")]
pub use dbb::*;

#[cfg(test)]
fn perft(position: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        let mut counter = MoveCounter::new();
        legal_moves(&position, &mut counter);
        return counter.moves as usize;
    }

    let mut moves = MoveVec::new();
    legal_moves(&position, &mut moves);

    let state = position.state().clone();
    let key = position.hash_key();
    let mut count = 0;
    for &mv in moves.iter() {
        let capture = position.make(mv);

        count += perft(position, depth - 1);

        position.unmake(mv, capture, &state, key);
    }

    count
}

#[test]
fn perft_test() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 3), 197281);
}

#[bench]
fn perft_bench_starting_position(b: &mut test::Bencher) {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    b.iter(|| -> usize { perft(&mut position, 2) });
}
