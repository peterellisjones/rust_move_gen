use square::Square;
use bb::BB;
use castle::Castle;

mod mv_counter;
mod mv_vec;

pub use self::mv_counter::MoveCounter;
pub use self::mv_vec::MoveVec;

/// MoveList represents a way to collect moves from move generation functions. Use this if you want to collect or record moves in a way not supported by MoveVec or MoveCounter
pub trait MoveList {
    /// Adds moves from the from-square. Targets is a bitboard of valid to-squares. Enemy is a bitboard of enemy pieces (ie pieces that can be captured)
    fn add_moves(&mut self, from: Square, targets: BB, enemy: BB);

    /// Adds the castle to the move list
    fn add_castle(&mut self, castle: Castle);

    /// Adds pawn non-captures to the list. Targets is a bitboard of valid to-squares. Shift is the distance the pawn moved to get to the target square, mod 64. For example, for a white piece moving forward one row this is '8'. For a black piece moving forward one row this is 56 (-8 % 64).
    fn add_pawn_pushes(&mut self, shift: usize, targets: BB);

    /// Adds pawn captures to list. Targets and shift are same as for `add_pawn_pushes`. Do not use this for en-passant captures (use `add_pawn_ep_capture`)
    fn add_pawn_captures(&mut self, shift: usize, targets: BB);

    /// Adds pawn en-passant capture to list. From and to are the squares the moving pieces moves from and to, respectively
    fn add_pawn_ep_capture(&mut self, from: Square, to: Square);
}
