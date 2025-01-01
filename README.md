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
