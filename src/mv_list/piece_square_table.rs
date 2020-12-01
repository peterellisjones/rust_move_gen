use castle::Castle;
use castle::*;
use piece::*;
use square::*;

/// PieceSquareTable gives scores for pieces on squares from the perspective
/// of white. See https://www.chessprogramming.org/Piece-Square_Tables
#[derive(Clone)]
pub struct PieceSquareTable {
  piece_square_values: [[i16; 64]; 6],
  castle_values: [i16; 2],
}

impl PieceSquareTable {
  /// piece_square_values should encode positional _and_ material value of
  /// piece on each square. eg if pawn is worth +100 material value and pawn on D4 is worth +20
  /// as positional value, entry for pawn on D4 should equal 100 + 20 = +120.
  pub fn new(piece_square_values: [[i16; 64]; 6]) -> PieceSquareTable {
    let mut castle_values = [0i16; 2];

    castle_values[QUEEN_SIDE.to_usize()] = piece_square_values[KING.to_usize()][C1.to_usize()]
      + piece_square_values[ROOK.to_usize()][D1.to_usize()]
      - piece_square_values[KING.to_usize()][E1.to_usize()]
      - piece_square_values[ROOK.to_usize()][A1.to_usize()];

    castle_values[KING_SIDE.to_usize()] = piece_square_values[KING.to_usize()][G1.to_usize()]
      + piece_square_values[ROOK.to_usize()][F1.to_usize()]
      - piece_square_values[KING.to_usize()][E1.to_usize()]
      - piece_square_values[ROOK.to_usize()][H1.to_usize()];

    PieceSquareTable {
      piece_square_values,
      castle_values,
    }
  }

  pub fn castle_score(&self, castle: Castle) -> i16 {
    unsafe { *self.castle_values.get_unchecked(castle.to_usize()) }
  }

  /// Score relative to white. Use Square#from_side to convert a square
  /// to the perspective of a given player (ie flip board if black)
  pub fn score(&self, kind: Kind, sq: Square) -> i16 {
    unsafe {
      *self
        .piece_square_values
        .get_unchecked(kind.to_usize())
        .get_unchecked(sq.to_usize())
    }
  }
}
