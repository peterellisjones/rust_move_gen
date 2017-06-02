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
#![feature(cfg_target_feature)]
#![feature(platform_intrinsics)]
#![feature(const_fn)]

pub mod bb;
mod position;
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
mod board;

#[cfg(target_feature = "sse3")]
mod dbb;

extern crate rand;
#[cfg(test)]
extern crate unindent;
#[cfg(test)]
extern crate test;

pub use position::{Position, State, STARTING_POSITION_FEN};
pub use board::Board;
pub use gen::{legal_moves, legal_captures};
pub use castle::{Castle, KING_SIDE, QUEEN_SIDE};
pub use castling_rights::{CastlingRights, BLACK_QS, BLACK_KS, WHITE_QS, WHITE_KS};
pub use mv::{Move, NULL_MOVE};
pub use mv_list::{MoveList, MoveCounter, MoveVec, MoveSlice};
pub use side::{Side, WHITE, BLACK};
pub use piece::*;
pub use square::*;
pub use bb::BB;

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
