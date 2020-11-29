use super::piece_square_table::PieceSquareTable;
use bb::{BB, EMPTY, END_ROWS};
use castle::Castle;
use mv::*;
use mv_list::MoveList;
use mv_list::CHECK_MATE_SCORE;
use piece::*;
use side::Side;
use square;
use square::Square;
use std::fmt;

/// BestMoveTracker implements MoveList and keeps a track of the best move seen so far
#[derive(Clone)]
pub struct BestMoveTracker<'a> {
  mv: Move,
  score: i16,
  piece_square_table: &'a PieceSquareTable,
  piece_grid: &'a [Piece; 64],
  stm: Side,
  count: usize, // counts the number of moves considered
}

impl<'a> BestMoveTracker<'a> {
  pub fn new(
    piece_square_table: &'a PieceSquareTable,
    piece_grid: &'a [Piece; 64],
    stm: Side,
  ) -> BestMoveTracker<'a> {
    BestMoveTracker {
      mv: NULL_MOVE,
      score: -CHECK_MATE_SCORE,
      piece_grid: piece_grid,
      piece_square_table: piece_square_table,
      stm: stm,
      count: 0,
    }
  }

  pub fn best(&self) -> MoveScore {
    MoveScore::new(self.mv, self.score)
  }

  pub fn score(&self) -> i16 {
    self.score
  }

  pub fn mv(&self) -> Move {
    self.mv
  }

  pub fn count(&self) -> usize {
    self.count
  }

  fn insert(&mut self, mv: Move, score: i16) {
    self.count += 1;
    if score > self.score {
      self.score = score;
      self.mv = mv;
    }
  }
}
impl<'a> fmt::Display for BestMoveTracker<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ({})", self.mv, self.score)
  }
}

impl<'a> fmt::Debug for BestMoveTracker<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ({})", self.mv, self.score)
  }
}

impl<'a> MoveList for BestMoveTracker<'a> {
  fn add_captures(&mut self, from: Square, targets: BB) {
    if targets == EMPTY {
      return;
    }

    let stm = self.stm;

    let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
    let from_score = self
      .piece_square_table
      .score(from_kind, from.from_side(stm));

    for (to, _) in targets.iter() {
      let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self
        .piece_square_table
        .score(capture_kind, to.from_side(stm.flip()));

      self.insert(
        Move::new_capture(from, to),
        -from_score + to_score + capture_score,
      );
    }
  }

  fn add_non_captures(&mut self, from: Square, targets: BB) {
    if targets == EMPTY {
      return;
    }

    let stm = self.stm;

    let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
    let from_score = self
      .piece_square_table
      .score(from_kind, from.from_side(stm));

    for (to, _) in targets.iter() {
      let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

      self.insert(Move::new_push(from, to), -from_score + to_score);
    }
  }

  fn add_castle(&mut self, castle: Castle) {
    let score = self.piece_square_table.castle_score(castle);
    self.insert(Move::new_castle(castle), score);
  }

  fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
    let stm = self.stm;
    let from_score = self.piece_square_table.score(PAWN, from.from_side(stm));
    let to_score = self.piece_square_table.score(PAWN, to.from_side(stm));
    // capture square is the file of the 'to' square
    // and the rank of the 'from' square
    let capture_sq = from.along_row_with_col(to);

    let capture_score = self
      .piece_square_table
      .score(PAWN, capture_sq.from_side(stm.flip()));

    let score = -from_score + to_score + capture_score;

    self.insert(Move::new_ep_capture(from, to), score);
  }

  fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
    let stm = self.stm;
    let from_kind = PAWN;

    for (to, _) in (targets & !END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self
        .piece_square_table
        .score(from_kind, from.from_side(stm));
      let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

      self.insert(Move::new_push(from, to), -from_score + to_score);
    }

    for (to, _) in (targets & END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self
        .piece_square_table
        .score(from_kind, from.from_side(stm));

      for to_kind in &[QUEEN, ROOK, BISHOP, KNIGHT] {
        let to_score = self.piece_square_table.score(*to_kind, to.from_side(stm));

        self.insert(
          Move::new_promotion(from, to, *to_kind),
          -from_score + to_score,
        );
      }
    }
  }

  fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
    let stm = self.stm;
    let from_kind = PAWN;

    for (to, _) in (targets & !END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self
        .piece_square_table
        .score(from_kind, from.from_side(stm));
      let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self
        .piece_square_table
        .score(capture_kind, to.from_side(stm.flip()));

      self.insert(
        Move::new_capture(from, to),
        -from_score + to_score + capture_score,
      );
    }

    for (to, _) in (targets & END_ROWS).iter() {
      let from = to.rotate_right(shift as square::Internal);
      let from_score = self
        .piece_square_table
        .score(from_kind, from.from_side(stm));

      let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
      let capture_score = self
        .piece_square_table
        .score(capture_kind, to.from_side(stm.flip()));

      for to_kind in &[QUEEN, ROOK, BISHOP, KNIGHT] {
        let to_score = self.piece_square_table.score(*to_kind, to.from_side(stm));

        self.insert(
          Move::new_capture_promotion(from, to, *to_kind),
          -from_score + to_score + capture_score,
        );
      }
    }
  }
}
