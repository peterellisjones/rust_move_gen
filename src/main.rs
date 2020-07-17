#![feature(test)]
#![feature(platform_intrinsics)]
#![feature(const_fn)]

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

pub use perft::perft;
pub use position::{Position, STARTING_POSITION_FEN};

use std::time::Instant;

fn main() {
  let position = &mut Position::from_fen(STARTING_POSITION_FEN).unwrap();

  let depth: usize = 6;
  println!(
    "Running performance test on starting position, depth {}",
    depth
  );
  let now = Instant::now();
  let move_count = perft(position, depth, false, 0);
  let elapsed = now.elapsed();
  let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
  let nps = move_count as f64 / sec;

  println!(
    "Done. Total moves: {} ({:5} seconds, {:0} NPS)",
    move_count, sec, nps
  );
}
