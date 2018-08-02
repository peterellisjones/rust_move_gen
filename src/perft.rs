use cache::Cache;
use gen::legal_moves;
use mv_list::{MoveCounter, MoveVec};
use num_cpus;
use position::Position;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn perft(
  position: &mut Position,
  depth: usize,
  multi_threading_enabled: bool,
  cache_bytes_per_thread: usize,
) -> usize {
  if depth == 0 {
    return 1;
  }

  if depth <= 3 {
    return perft_inner(position, depth);
  }

  if !multi_threading_enabled {
    if cache_bytes_per_thread > 0 {
      let mut cache = Cache::new(cache_bytes_per_thread).unwrap();
      return perft_with_cache_inner(position, depth, &mut cache);
    } else {
      return perft_inner(position, depth);
    }
  }

  let pool = ThreadPool::new(num_cpus::get());
  let (tx, rx) = channel();

  let mut moves = MoveVec::new();
  legal_moves(&position, &mut moves);
  let moves_len = moves.len();

  for &mv in moves.iter() {
    let tx = tx.clone();
    let mut position_local = position.clone();

    pool.execute(move || {
      position_local.make(mv);

      let count: usize;
      if cache_bytes_per_thread > 0 {
        let mut cache = Cache::new(cache_bytes_per_thread).unwrap();
        count = perft_with_cache_inner(&mut position_local, depth - 1, &mut cache);
      } else {
        count = perft_inner(&mut position_local, depth - 1);
      }

      tx.send(count).unwrap();
    });
  }

  return rx.iter().take(moves_len).sum();
}

pub fn perft_inner(position: &mut Position, depth: usize) -> usize {
  if depth == 1 {
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

    count += perft_inner(position, depth - 1);

    position.unmake(mv, capture, &state, key);
  }

  count
}

fn perft_with_cache_inner(position: &mut Position, depth: usize, cache: &mut Cache) -> usize {
  let key = position.hash_key();

  let ret = cache.probe(key, depth);
  if ret.is_some() {
    return ret.unwrap();
  }

  let mut count = 0;
  if depth == 1 {
    let mut counter = MoveCounter::new();
    legal_moves(&position, &mut counter);
    count = counter.moves as usize;
  } else {
    let mut moves = MoveVec::new();
    legal_moves(&position, &mut moves);

    let state = position.state().clone();
    let key = position.hash_key();
    for &mv in moves.iter() {
      let capture = position.make(mv);

      count += perft_with_cache_inner(position, depth - 1, cache);

      position.unmake(mv, capture, &state, key);
    }
  }

  cache.save(key, count, depth as i16);

  count
}

#[cfg(test)]
mod test {
  use super::*;
  use position::{Position, STARTING_POSITION_FEN};
  use test;

  #[test]
  fn perft_test_3() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 3, false, 0), 8902);
  }

  #[test]
  fn perft_test_4() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 4, false, 0), 197281);
  }

  #[test]
  fn perft_with_cache_test_3() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 3, false, 1024 * 1024), 8902);
  }

  #[test]
  fn perft_with_cache_test_4() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 4, false, 1024 * 1024), 197281);
  }

  #[bench]
  fn perft_bench_starting_position(b: &mut test::Bencher) {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    b.iter(|| -> usize { perft(&mut position, 2, false, 0) });
  }
}
