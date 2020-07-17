use bb::{BB, END_ROWS};
use castle::Castle;
use mv::Move;
use mv_list::MoveList;
use piece::*;
use side::{Side, WHITE};
use square;
use square::Square;
use std;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

#[derive(Clone, Eq, Copy)]
pub struct MoveScore(Move, i16);

impl Ord for MoveScore {
  fn cmp(&self, other: &Self) -> Ordering {
    self.1.cmp(&other.1)
  }
}

impl PartialOrd for MoveScore {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.1.cmp(&other.1))
  }
}

impl PartialEq for MoveScore {
  fn eq(&self, other: &Self) -> bool {
    self.1 == other.1
  }
}

/// SortedScoredMoveList is list move vec but calculates the piece-square score of each move as it adds them to the list
/// This is more efficient than calculating scores later as it results in fewer lookups into the piece-square table
/// Underlying structure is a binary heap which allows O(1) insertion and fast ordered interation via into_iter()
#[derive(Clone)]
pub struct SortedScoredMoveList<'a> {
  moves: BinaryHeap<MoveScore>,
  piece_square_values: &'a [[i16; 64]; 6],
  castle_values: &'a [i16; 2],
  piece_grid: &'a [Piece; 64],
  stm: Side,
}

impl<'a> fmt::Display for SortedScoredMoveList<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl<'a> fmt::Debug for SortedScoredMoveList<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl<'a> MoveList for SortedScoredMoveList<'a> {
  fn add_moves(&mut self, from: Square, targets: BB, enemy: BB) {
    self.insert_non_captures(from, targets & (!enemy));
    self.insert_captures(from, targets & enemy);
  }

  fn add_castle(&mut self, castle: Castle) {
    let score = unsafe { *self.castle_values.get_unchecked(castle.to_usize()) };

    self.insert(Move::new_castle(castle), score);
  }

  fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
    let idx_mod = if self.stm == WHITE { 0 } else { 56 };
    let from_score = self.piece_square_score(PAWN, from, idx_mod);
    let to_score = self.piece_square_score(PAWN, to, idx_mod);
    // capture square is the file of the 'to' square
    // and the rank of the 'from' square
    let capture_sq = from.along_row_with_col(to);
    let capture_score = self.piece_square_score(PAWN, capture_sq, idx_mod ^ 56);

    let score = -from_score + to_score + capture_score;

    self.insert(Move::new_ep_capture(from, to), score);
  }

  fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
    let idx_mod = if self.stm == WHITE { 0 } else { 56 };
    let from_kind = PAWN;

    for (to, _) in (targets & !END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self.piece_square_score(from_kind, from, idx_mod);
      let to_score = self.piece_square_score(from_kind, to, idx_mod);

      self.insert(Move::new_push(from, to), -from_score + to_score);
    }

    for (to, _) in (targets & END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self.piece_square_score(from_kind, from, idx_mod);

      for to_kind in &[QUEEN, ROOK, BISHOP, KNIGHT] {
        let to_score = self.piece_square_score(*to_kind, to, idx_mod);

        self.insert(Move::new_push(from, to), -from_score + to_score);
      }
    }
  }

  fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
    let idx_mod = if self.stm == WHITE { 0 } else { 56 };
    let from_kind = PAWN;

    for (to, _) in (targets & !END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self.piece_square_score(from_kind, from, idx_mod);
      let to_score = self.piece_square_score(from_kind, to, idx_mod);

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self.piece_square_score(capture_kind, to, idx_mod ^ 56);

      self.insert(
        Move::new_capture(from, to),
        -from_score + to_score + capture_score,
      );
    }

    for (to, _) in (targets & END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self.piece_square_score(from_kind, from, idx_mod);

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self.piece_square_score(capture_kind, to, idx_mod ^ 56);

      for to_kind in &[QUEEN, ROOK, BISHOP, KNIGHT] {
        let to_score = self.piece_square_score(*to_kind, to, idx_mod);

        self.insert(
          Move::new_capture(from, to),
          -from_score + to_score + capture_score,
        );
      }
    }
  }
}

impl<'a> SortedScoredMoveList<'a> {
  #[allow(dead_code)]
  pub fn new(
    piece_square_values: &'a [[i16; 64]; 6],
    castle_values: &'a [i16; 2],
    piece_grid: &'a [Piece; 64],
    stm: Side,
  ) -> SortedScoredMoveList<'a> {
    SortedScoredMoveList {
      moves: BinaryHeap::<MoveScore>::new(),
      piece_square_values: piece_square_values,
      castle_values: castle_values,
      piece_grid: piece_grid,
      stm: stm,
    }
  }

  pub fn to_string(&self) -> String {
    self
      .iter_unsorted()
      .map(|pair: &MoveScore| format!("{} ({})", pair.0, pair.1))
      .collect::<Vec<String>>()
      .join(", ")
  }

  pub fn iter_unsorted(&self) -> std::collections::binary_heap::Iter<MoveScore> {
    self.moves.iter()
  }

  #[allow(dead_code)]
  pub fn into_iter(self) -> std::collections::binary_heap::IntoIterSorted<MoveScore> {
    self.moves.into_iter_sorted()
  }

  fn insert_non_captures(&mut self, from: Square, targets: BB) {
    let idx_mod = if self.stm == WHITE { 0 } else { 56 };

    let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
    let from_score = self.piece_square_score(from_kind, from, idx_mod);

    for (to, _) in targets.iter() {
      let to_score = self.piece_square_score(from_kind, to, idx_mod);

      self.insert(Move::new_push(from, to), -from_score + to_score);
    }
  }

  fn insert_captures(&mut self, from: Square, targets: BB) {
    let idx_mod = if self.stm == WHITE { 0 } else { 56 };

    let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
    let from_score = self.piece_square_score(from_kind, from, idx_mod);

    for (to, _) in targets.iter() {
      let to_score = self.piece_square_score(from_kind, to, idx_mod);

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self.piece_square_score(capture_kind, to, idx_mod ^ 56);

      self.insert(
        Move::new_capture(from, to),
        -from_score + to_score + capture_score,
      );
    }
  }

  #[inline]
  fn insert(&mut self, mv: Move, score: i16) {
    self.moves.push(MoveScore(mv, score));
  }

  fn piece_square_score(&self, kd: Kind, sq: Square, idx_mod: usize) -> i16 {
    debug_assert!(idx_mod == 0 || idx_mod == 56);

    return unsafe {
      *self
        .piece_square_values
        .get_unchecked(kd.to_usize())
        .get_unchecked(sq.to_usize() ^ idx_mod)
    };
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize {
    self.moves.len()
  }
}

#[cfg(test)]
mod test {
  extern crate rand;

  use super::*;
  use gen::*;
  use position::*;
  use rand::seq::SliceRandom;
  use rand::Rng;
  use square::*;

  #[test]
  fn test_scored_move_vec() {
    let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let piece_square_values = [[100i16; 64]; 6];
    let castle_values = [0i16; 2];

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_eq!(list.len(), 20);
  }

  #[test]
  fn test_ordering() {
    let position = &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/4K3 w kq - 0 1").unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];

    // value of king on F2 to +123
    piece_square_values[KING.to_usize()][F2.to_usize()] = 123i16;

    // value of king on D1 to +99
    piece_square_values[KING.to_usize()][D1.to_usize()] = 99i16;

    let castle_values = [0i16; 2];

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_eq!(list.len(), 5);

    // check moves are sorted by score descending
    let sorted_vec = list.into_iter().collect::<Vec<MoveScore>>();

    // first move should be k e1f2 +23
    let move_1 = sorted_vec[0].0;
    let score_1 = sorted_vec[0].1;
    assert_eq!(move_1.to_string(), "e1f2");
    assert_eq!(score_1, 23i16);

    // last move should be k e1d1 -1
    let move_5 = sorted_vec[4].0;
    let score_5 = sorted_vec[4].1;
    assert_eq!(move_5.to_string(), "e1d1");
    assert_eq!(score_5, -1i16);
  }

  #[test]
  fn test_white_castle_scoring() {
    let position = &Position::from_fen("rnbqkbnr/8/8/8/8/8/8/R3K2R w K").unwrap();
    let piece_square_values = [[100i16; 64]; 6];
    let castle_values = [123i16, 456i16];

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&list, &["O-O (456)"]);
  }

  #[test]
  fn test_black_castle_scoring() {
    let position = &Position::from_fen("r3kbnr/8/8/8/8/8/8/R3K2R b KQq").unwrap();
    let piece_square_values = [[100i16; 64]; 6];
    let castle_values = [123i16, 456i16];

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&list, &["O-O-O (123)"]);
  }

  #[test]
  fn test_pawn_push_scoring() {
    let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];
    let castle_values = [0i16; 2];

    // make Pawn C2 150, Pawn C4 165. Should have move worth +15
    piece_square_values[PAWN.to_usize()][C2.to_usize()] = 150;
    piece_square_values[PAWN.to_usize()][C4.to_usize()] = 165;

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&list, &["c2c4 (15)"]);
  }

  #[test]
  fn test_push_scoring() {
    let position =
      &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b QqKk - 0 1").unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];
    let castle_values = [0i16; 2];

    // make knight B1 300, C3 333. Should have move worth +33
    piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
    piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&list, &["b8c6 (33)"]);
  }
  #[test]
  fn test_capture_scoring() {
    // white parn on C3
    let position =
      &Position::from_fen("rnbqkbnr/pppppppp/2P5/8/8/8/PP1PPPPP/RNBQKBNR b QqKk - 0 1").unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];
    let castle_values = [0i16; 2];

    // make knight B1 300, C3 333. Should have move worth +33
    piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
    piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

    // pawn on C6 worth 50
    piece_square_values[PAWN.to_usize()][C6.to_usize()] = 50;

    let mut list = SortedScoredMoveList::new(
      &piece_square_values,
      &castle_values,
      position.grid(),
      position.state().stm,
    );

    legal_moves(&position, &mut list);

    println!("{}", list);

    assert_list_includes_moves(&list, &["b8xc6 (83)"]);
  }

  #[test]
  fn test_integrity() {
    // makes 20 random moves, and checks that adding the move scores adds up to the
    // final position score
    let position = &mut Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let (piece_square_values, castle_values) = random_piece_square_values();

    let initial_score = score_for_position(position, &piece_square_values);

    let mut moves_scores = 0i16;

    for _ in 0..20 {
      // note must be even number of iterations to ensure final score is calculated from WHITE's PoV
      let stm = position.state().stm;

      let move_score = {
        let mut list =
          SortedScoredMoveList::new(&piece_square_values, &castle_values, position.grid(), stm);

        legal_moves(&position, &mut list);

        let sorted_vec = list.into_iter().collect::<Vec<MoveScore>>();

        *sorted_vec.choose(&mut rand::thread_rng()).unwrap()
      };

      let score = move_score.1;
      let mv = move_score.0;

      moves_scores += if stm == WHITE { score } else { -score };

      position.make(mv);
    }

    let score_after_moves = score_for_position(position, &piece_square_values);

    assert_eq!(initial_score + moves_scores, score_after_moves);
  }

  fn assert_list_includes_moves(list: &SortedScoredMoveList, moves: &[&'static str]) {
    for &m in moves.iter() {
      assert!(list
        .iter_unsorted()
        .map(|pair: &MoveScore| format!("{} ({})", pair.0, pair.1))
        .any(|mv| mv == m));
    }
  }

  fn random_piece_square_values() -> ([[i16; 64]; 6], [i16; 2]) {
    let mut piece_square_values = [[100i16; 64]; 6];

    for pc in 0..6 {
      for sq in 0..64 {
        piece_square_values[pc][sq] = rand::thread_rng().gen_range(100, 200) as i16;
      }
    }

    let castle_values = [
      // Queenside
      piece_square_values[KING.to_usize()][C1.to_usize()]
        + piece_square_values[ROOK.to_usize()][D1.to_usize()]
        - piece_square_values[KING.to_usize()][E1.to_usize()]
        - piece_square_values[ROOK.to_usize()][A1.to_usize()],
      // Kingside
      piece_square_values[KING.to_usize()][G1.to_usize()]
        + piece_square_values[ROOK.to_usize()][F1.to_usize()]
        - piece_square_values[KING.to_usize()][E1.to_usize()]
        - piece_square_values[ROOK.to_usize()][H1.to_usize()],
    ];

    (piece_square_values, castle_values)
  }

  fn score_for_position(pos: &Position, piece_square_values: &[[i16; 64]; 6]) -> i16 {
    let mut score = 0i16;

    for (idx, &pc) in pos.grid().iter().enumerate() {
      if pc.is_some() {
        let sq = if pc.side() == WHITE { idx } else { idx ^ 56 };
        let piece_square_value = piece_square_values[pc.kind().to_usize()][sq];

        score += if pos.state().stm == pc.side() {
          piece_square_value
        } else {
          -piece_square_value
        }
      }
    }

    score
  }
}
