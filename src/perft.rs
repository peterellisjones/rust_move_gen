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
