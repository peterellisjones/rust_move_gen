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

pub use crate::position::{Position, STARTING_POSITION_FEN};
pub use perft::perft;

use std::time::Instant;
fn main() {
    let fen = STARTING_POSITION_FEN;
    let mut position = Position::from_fen(fen).unwrap();

    let depth: usize = 7;
    println!(
        "Running performance test on starting position, depth {}",
        depth
    );
    let now = Instant::now();
    let move_count = perft(&mut position, depth, true, 1024 * 1024 * 4);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
    let nps = move_count as f64 / sec;

    println!(
        "Done. Total moves: {} ({:5} seconds, {:0} NPS)",
        move_count, sec, nps
    );
}
