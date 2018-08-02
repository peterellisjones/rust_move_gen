use cache::Cache;
use gen::legal_moves;
use mv_list::{MoveCounter, MoveVec};
use num_cpus;
use position::Position;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn perft(position: &mut Position, depth: usize) -> usize {
  if depth == 0 {
    return 1;
  }

  if depth <= 2 {
    return perft_inner(position, depth);
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
      tx.send(perft_inner(&mut position_local, depth - 1))
        .unwrap();
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

pub fn perft_with_cache(position: &mut Position, depth: usize, cache_size_bytes: usize) -> usize {
  if depth == 0 {
    return 1;
  }

  let mut cache = Cache::new(cache_size_bytes).unwrap();

  return perft_with_cache_inner(position, depth, &mut cache);
}

fn perft_with_cache_inner(position: &mut Position, depth: usize, cache: &mut Cache) -> usize {
  let key = position.hash_key();

  let ret = cache.probe(key, depth);
  if ret.is_some() {
    return ret.unwrap();
  }

  if depth == 1 {
    let mut counter = MoveCounter::new();
    legal_moves(&position, &mut counter);
    let count = counter.moves as usize;
    cache.save(key, count, depth as i16);

    return count;
  }

  let mut moves = MoveVec::new();
  legal_moves(&position, &mut moves);

  let state = position.state().clone();
  let key = position.hash_key();
  let mut count = 0;
  for &mv in moves.iter() {
    let capture = position.make(mv);

    count += perft_with_cache_inner(position, depth - 1, cache);

    position.unmake(mv, capture, &state, key);
  }

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

    assert_eq!(perft(&mut position, 3), 8902);
  }

  #[test]
  fn perft_test_4() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft(&mut position, 4), 197281);
  }

  #[test]
  fn perft_with_cache_test_3() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft_with_cache(&mut position, 3, 1024 * 1024), 8902);
  }

  #[test]
  fn perft_with_cache_test_4() {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    assert_eq!(perft_with_cache(&mut position, 4, 1024 * 1024), 197281);
  }

  #[bench]
  fn perft_bench_starting_position(b: &mut test::Bencher) {
    let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

    b.iter(|| -> usize { perft(&mut position, 2) });
  }
}
