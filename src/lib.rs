//! # chess_move_gen
//! Provides structs and methods for generating chess moves efficiently
//!
//! Example usage:
//!
//! ```
//! let mut list = MoveVec::new();
//! let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1").unwrap();
//! legal_moves::<MoveVec>(board, &mut list);
//! println!("{}", list);
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
mod dbb;
mod util;
mod hash;

extern crate rand;
#[cfg(test)]
extern crate unindent;
#[cfg(test)]
extern crate test;

#[cfg(target_feature = "sse3")]
extern crate simd;

pub use board::{Board, State, STARTING_POSITION_FEN};
pub use gen::legal_moves;
pub use castle::{Castle, KING_SIDE, QUEEN_SIDE};
pub use castling_rights::*;
pub use mv::Move;
pub use mv_list::{MoveList, MoveCounter, MoveVec};
pub use side::*;
pub use piece::*;
pub use square::*;
pub use bb::*;
pub use dbb::*;