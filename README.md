Fast chess move generation library. Uses SIMD for fast sliding piece move generation

Example usage:

```
use chess_move_gen::*;
let mut list = MoveVec::new();
let position = &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1").unwrap();
legal_moves::<MoveVec>(position, &mut list);
assert_eq!(list.len(), 20);
```

Ways to store moves:

`MoveVec`: Wraps a vector of generated moves, useful if you need to access the actual moves being generated

`MoveCounter`: Counts moves of each kind (captures, castles, promotions etc). Useful if you are making a perft function or need statistics about moves for a position, but don't care about the actual moves

`SortedMoveAdder` + `SortedMoveHeap`: Stores genarated moves in a sorted binary heap, which are efficiently ordered as they are inserted based on a heuristic scoring and piece-square table that you provide. Use this if you want the moves to have a reasonably good initial ordering so moves that are checked first are more likely to lead to eg alpha-beta cutoffs and reduce the search tree size.

---

This crate also provides many Rust implementations for [sliding-piece move generation](https://www.chessprogramming.org/Move_Generation). Run `RUSTFLAGS='-C target-cpu=native' cargo bench` to see the performance of each implementation on your machine.

```
test generation::slider::ray_bmi2::test::bench_bishop_attacks_from_sq         ... bench:         529.68 ns/iter (+/- 132.89)
test generation::slider::ray_bmi2::test::bench_multiple_bishop_attacks        ... bench:       1,523.23 ns/iter (+/- 271.93)
test generation::slider::ray_bmi2::test::bench_multiple_rook_attacks          ... bench:       1,523.64 ns/iter (+/- 138.77)
test generation::slider::ray_bmi2::test::bench_rook_attacks_from_sq           ... bench:         652.55 ns/iter (+/- 63.71)
test generation::slider::ray_hyperbola::test::bench_bishop_attacks_from_sq    ... bench:         895.29 ns/iter (+/- 199.15)
test generation::slider::ray_hyperbola::test::bench_multiple_bishop_attacks   ... bench:       2,489.84 ns/iter (+/- 214.09)
test generation::slider::ray_hyperbola::test::bench_multiple_rook_attacks     ... bench:       2,974.33 ns/iter (+/- 298.86)
test generation::slider::ray_hyperbola::test::bench_rook_attacks_from_sq      ... bench:         859.65 ns/iter (+/- 116.43)
test generation::slider::ray_kogge_stone::test::bench_bishop_attacks_from_sq  ... bench:       1,336.16 ns/iter (+/- 107.25)
test generation::slider::ray_kogge_stone::test::bench_multiple_bishop_attacks ... bench:       1,272.63 ns/iter (+/- 318.64)
test generation::slider::ray_kogge_stone::test::bench_multiple_rook_attacks   ... bench:       1,225.76 ns/iter (+/- 102.96)
test generation::slider::ray_kogge_stone::test::bench_rook_attacks_from_sq    ... bench:       1,296.88 ns/iter (+/- 41.14)
test generation::slider::ray_naive::test::bench_bishop_attacks_from_sq        ... bench:       2,358.44 ns/iter (+/- 225.09)
test generation::slider::ray_naive::test::bench_multiple_bishop_attacks       ... bench:      15,144.05 ns/iter (+/- 691.27)
test generation::slider::ray_naive::test::bench_multiple_rook_attacks         ... bench:      15,526.90 ns/iter (+/- 903.83)
test generation::slider::ray_naive::test::bench_rook_attacks_from_sq          ... bench:       2,311.20 ns/iter (+/- 215.17)
test generation::slider::ray_subtract::test::bench_bishop_attacks_from_sq     ... bench:       1,901.60 ns/iter (+/- 324.79)
test generation::slider::ray_subtract::test::bench_rook_attacks_from_sq       ... bench:       2,082.14 ns/iter (+/- 624.36)
```
