use bb::BB;
use castle::Castle;
use square::Square;

mod best_mv_tracker;
mod mv_counter;
mod mv_vec;
mod piece_square_table;
mod scored_mv_list;

pub use self::best_mv_tracker::BestMoveTracker;
pub use self::mv_counter::MoveCounter;
pub use self::mv_vec::MoveVec;
pub use self::piece_square_table::PieceSquareTable;
pub use self::scored_mv_list::ScoredMoveList;

pub const CHECK_MATE_SCORE: i16 = std::i16::MAX / 2;

/// MoveList represents a way to collect moves from move generation functions. Use this if you want to collect or record moves in a way not supported by MoveVec or MoveCounter
pub trait MoveList {
  fn add_captures(&mut self, from: Square, targets: BB);
  fn add_non_captures(&mut self, from: Square, targets: BB);

  /// Adds the castle to the move list
  fn add_castle(&mut self, castle: Castle);

  /// Adds pawn non-captures to the list. Targets is a bitboard of valid to-squares. Shift is the distance the pawn moved to get to the target square, mod 64. For example, for a white piece moving forward one row this is '8'. For a black piece moving forward one row this is 56 (-8 % 64).
  fn add_pawn_pushes(&mut self, shift: usize, targets: BB);

  /// Adds pawn captures to list. Targets and shift are same as for `add_pawn_pushes`. Do not use this for en-passant captures (use `add_pawn_ep_capture`)
  fn add_pawn_captures(&mut self, shift: usize, targets: BB);

  /// Adds pawn en-passant capture to list. From and to are the squares the moving pieces moves from and to, respectively
  fn add_pawn_ep_capture(&mut self, from: Square, to: Square);
}
