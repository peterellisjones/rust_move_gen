use super::piece_square_table::PieceSquareTable;
use bb::{BB, EMPTY, END_ROWS};
use castle::Castle;
use mv::{Move, MoveScore};
use mv_list::MoveList;
use piece::*;
use side::Side;
use square;
use square::*;
use std::fmt;

/// ScoredMoveList is list move vec but calculates the piece-square score of each move as it adds them to the list
/// This is more efficient than calculating scores later
pub struct ScoredMoveList<'a> {
  moves: &'a mut Vec<MoveScore>,
  piece_square_table: &'a PieceSquareTable,
  piece_grid: &'a [Piece; 64],
  stm: Side,
}

impl<'a> fmt::Display for ScoredMoveList<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl<'a> fmt::Debug for ScoredMoveList<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl<'a> MoveList for ScoredMoveList<'a> {
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

impl<'a> ScoredMoveList<'a> {
  pub fn new(
    piece_square_table: &'a PieceSquareTable,
    piece_grid: &'a [Piece; 64],
    stm: Side,
    moves: &'a mut Vec<MoveScore>,
  ) -> ScoredMoveList<'a> {
    ScoredMoveList {
      moves: moves,
      piece_square_table: piece_square_table,
      piece_grid: piece_grid,
      stm: stm,
    }
  }

  pub fn to_string(&self) -> String {
    self
      .moves
      .iter()
      .map(|pair: &MoveScore| format!("{} ({})", pair.mv(), pair.score()))
      .collect::<Vec<String>>()
      .join(", ")
  }

  pub fn moves(&self) -> &Vec<MoveScore> {
    &self.moves
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize {
    self.moves.len()
  }

  fn insert(&mut self, mv: Move, score: i16) {
    self.moves.push(MoveScore::new(mv, score));
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
  use side::WHITE;

  #[test]
  fn test_scored_move_vec() {
    let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let piece_square_table = PieceSquareTable::new([[100i16; 64]; 6]);

    let mut vec: Vec<MoveScore> = Vec::with_capacity(60);
    let mut list = ScoredMoveList::new(
      &piece_square_table,
      position.grid(),
      position.state().stm,
      &mut vec,
    );

    legal_moves(&position, &mut list);

    assert_eq!(vec.len(), 20);
  }

  // #[test]
  // fn test_white_castle_scoring() {
  //     let position = &Position::from_fen("rnbqkbnr/8/8/8/8/8/8/R3K2R w K").unwrap();
  //     let piece_square_table = PieceSquareTable::new([[100i16; 64]; 6]);

  //     let mut list =
  //         ScoredMoveList::new(&piece_square_table, position.grid(), position.state().stm);

  //     legal_moves(&position, &mut list);

  //     assert_list_includes_moves(&list, &["O-O (456)"]);
  // }

  // #[test]
  // fn test_black_castle_scoring() {
  //     let position = &Position::from_fen("r3kbnr/8/8/8/8/8/8/R3K2R b KQq").unwrap();
  //     let piece_square_table = PieceSquareTable::new([[100i16; 64]; 6]);

  //     let mut list =
  //         ScoredMoveList::new(&piece_square_table, position.grid(), position.state().stm);

  //     legal_moves(&position, &mut list);

  //     assert_list_includes_moves(&list, &["O-O-O (123)"]);
  // }

  #[test]
  fn test_pawn_push_scoring() {
    let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];

    // make Pawn C2 150, Pawn C4 165. Should have move worth +15
    piece_square_values[PAWN.to_usize()][C2.to_usize()] = 150;
    piece_square_values[PAWN.to_usize()][C4.to_usize()] = 165;

    let piece_square_table = PieceSquareTable::new(piece_square_values);
    let mut vec: Vec<MoveScore> = Vec::with_capacity(60);
    let mut list = ScoredMoveList::new(
      &piece_square_table,
      position.grid(),
      position.state().stm,
      &mut vec,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&vec, &["c2c4 (15)"]);
  }

  #[test]
  fn test_push_scoring() {
    let position =
      &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b QqKk - 0 1").unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];

    // make knight B1 300, C3 333. Should have move worth +33
    piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
    piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

    let piece_square_table = PieceSquareTable::new(piece_square_values);
    let mut vec: Vec<MoveScore> = Vec::with_capacity(60);
    let mut list = ScoredMoveList::new(
      &piece_square_table,
      position.grid(),
      position.state().stm,
      &mut vec,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&vec, &["b8c6 (33)"]);
  }
  #[test]
  fn test_capture_scoring() {
    // white pawn on C3
    let position =
      &Position::from_fen("rnbqkbnr/pppppppp/2P5/8/8/8/PP1PPPPP/RNBQKBNR b QqKk - 0 1").unwrap();
    let mut piece_square_values = [[100i16; 64]; 6];

    // make knight B1 300, C3 333. Should have move worth +33
    piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
    piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

    // pawn on C6 worth 50
    piece_square_values[PAWN.to_usize()][C6.to_usize()] = 50;

    let piece_square_table = PieceSquareTable::new(piece_square_values);
    let mut vec: Vec<MoveScore> = Vec::with_capacity(60);
    let mut list = ScoredMoveList::new(
      &piece_square_table,
      position.grid(),
      position.state().stm,
      &mut vec,
    );

    legal_moves(&position, &mut list);

    assert_list_includes_moves(&vec, &["b8xc6 (83)"]);
  }

  #[test]
  fn test_integrity() {
    // makes 30 random moves, and checks that adding the move scores adds up to the
    // final position score
    let position = &mut Position::from_fen(STARTING_POSITION_FEN).unwrap();
    let piece_square_table = random_piece_square_table();

    let initial_score = score_for_position(position, &piece_square_table);

    let mut moves_scores = 0i16;

    for _ in 0..30 {
      // note must be even number of iterations to ensure final score is calculated from WHITE's PoV
      let stm = position.state().stm;

      let move_score = {
        let mut vec: Vec<MoveScore> = Vec::with_capacity(60);
        let mut list = ScoredMoveList::new(
          &piece_square_table,
          position.grid(),
          position.state().stm,
          &mut vec,
        );

        legal_moves(&position, &mut list);

        *vec.choose(&mut rand::thread_rng()).unwrap()
      };

      let score = move_score.score();
      let mv = move_score.mv();

      moves_scores += if stm == WHITE { score } else { -score };

      position.make(mv);
    }

    let score_after_moves = score_for_position(position, &piece_square_table);

    assert_eq!(initial_score + moves_scores, score_after_moves);
  }

  fn assert_list_includes_moves(list: &Vec<MoveScore>, moves: &[&'static str]) {
    for &m in moves.iter() {
      assert!(list
        .iter()
        .map(|pair: &MoveScore| format!("{} ({})", pair.mv(), pair.score()))
        .any(|mv| mv == m));
    }
  }

  fn random_piece_square_table() -> PieceSquareTable {
    let mut piece_square_values = [[100i16; 64]; 6];

    for pc in 0..6 {
      for sq in 0..64 {
        piece_square_values[pc][sq] = rand::thread_rng().gen_range(100, 200) as i16;
      }
    }

    PieceSquareTable::new(piece_square_values)
  }

  fn score_for_position(pos: &Position, piece_square_table: &PieceSquareTable) -> i16 {
    let mut score = 0i16;

    for (idx, &pc) in pos.grid().iter().enumerate() {
      if pc.is_some() {
        let piece_square_value =
          piece_square_table.score(pc.kind(), Square(idx).from_side(pc.side()));

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
