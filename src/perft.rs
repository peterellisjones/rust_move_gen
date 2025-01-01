use crate::generation::legal_moves;
use crate::mv_list::{MoveCounter, MoveVec};
use crate::position::Position;
use crate::cache::Cache;
use num_cpus;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

/// Returns the number of nodes at the provided depth
/// cache_bytes_per_thread must be of form 2^N bytes
/// if multi_threading_enabled is set to true search will
/// run concurrently accross threads equal to your CPU count
pub fn perft(
    position: &mut Position,
    depth: usize,
    multi_threading_enabled: bool,
    cache_bytes_per_thread: usize,
) -> u64 {
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

            let count: u64;
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

pub fn perft_inner(position: &mut Position, depth: usize) -> u64 {
    if depth == 1 {
        let mut counter = MoveCounter::new();
        legal_moves(&position, &mut counter);
        return counter.moves;
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

fn perft_with_cache_inner(position: &mut Position, depth: usize, cache: &mut Cache) -> u64 {
    let key = position.hash_key();

    let result = cache.probe(key, depth);
    if let Some(value) = result {
        return value;
    }

    let mut count = 0;
    if depth == 1 {
        let mut counter = MoveCounter::new();
        legal_moves(&position, &mut counter);
        count = counter.moves as u64;
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

/// Returns the number of moves, captures, promotions, castles and
/// en-passant captures at the provided depth
/// if multi_threading_enabled is set to true search will
/// run concurrently accross threads equal to your CPU count
#[allow(dead_code)]
pub fn perft_detailed(
    position: &mut Position,
    depth: usize,
    multi_threading_enabled: bool,
) -> MoveCounter {
    if depth == 0 {
        return MoveCounter::new();
    }

    if depth <= 3 {
        return perft_detailed_inner(position, depth);
    }

    if !multi_threading_enabled {
        return perft_detailed_inner(position, depth);
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

            let counter: MoveCounter;
            counter = perft_detailed_inner(&mut position_local, depth - 1);

            tx.send(counter).unwrap();
        });
    }

    let mut counter = MoveCounter::new();
    for c in rx.iter().take(moves_len) {
        counter += c;
    }

    counter
}

#[allow(dead_code)]
pub fn perft_detailed_inner(position: &mut Position, depth: usize) -> MoveCounter {
    let mut counter = MoveCounter::new();

    if depth == 1 {
        legal_moves(&position, &mut counter);
        return counter;
    }

    let mut moves = MoveVec::new();
    legal_moves(&position, &mut moves);

    let state = position.state().clone();
    let key = position.hash_key();
    for &mv in moves.iter() {
        let capture = position.make(mv);

        counter += perft_detailed_inner(position, depth - 1);

        position.unmake(mv, capture, &state, key);
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::position::{Position, STARTING_POSITION_FEN};
    use ::test;

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

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_2
    fn perft_detailed_position_2_depth_3() {
        let mut position =
            Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
                .unwrap();
        let counter = perft_detailed(&mut position, 3, true);

        assert_eq!(counter.promotions, 0);
        assert_eq!(counter.castles, 3162);
        assert_eq!(counter.ep_captures, 45);
        assert_eq!(counter.captures, 17102);
        assert_eq!(counter.moves, 97862);
    }

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_2
    fn perft_detailed_position_2_depth_4() {
        let mut position =
            Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
                .unwrap();
        let counter = perft_detailed(&mut position, 4, true);

        assert_eq!(counter.promotions, 15172);
        assert_eq!(counter.castles, 128013);
        assert_eq!(counter.ep_captures, 1929);
        assert_eq!(counter.captures, 757163);
        assert_eq!(counter.moves, 4085603);
    }

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_3
    fn perft_position_3_depth_5() {
        let mut position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();

        assert_eq!(perft(&mut position, 5, true, 1024 * 1024), 674624);
    }

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_4
    fn perft_position_4_depth_4() {
        let mut position =
            Position::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1")
                .unwrap();

        assert_eq!(perft(&mut position, 4, true, 1024 * 1024), 422333);
    }

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_5
    fn perft_position_5_depth_3() {
        let mut position =
            Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                .unwrap();

        assert_eq!(perft(&mut position, 3, true, 1024 * 1024), 62379);
    }

    #[test]
    // https://www.chessprogramming.org/Perft_Results#Position_5
    fn perft_debug_1() {
        let mut position =
            Position::from_fen("r3k2r/p1pp1pb1/bn2pqp1/3PN3/1p2P3/2N5/PPPBBPpP/R4K1R w kq - 0 1")
                .unwrap();

        assert_eq!(perft(&mut position, 1, false, 0), 3);
    }

    #[bench]
    fn perft_bench_starting_position(b: &mut test::Bencher) {
        let mut position = Position::from_fen(STARTING_POSITION_FEN).unwrap();

        b.iter(|| -> u64 { perft(&mut position, 2, false, 0) });
    }
}
