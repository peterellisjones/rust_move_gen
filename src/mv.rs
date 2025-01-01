use crate::castle::{Castle, KING_SIDE, QUEEN_SIDE};
use crate::piece::*;
use crate::square::{Square, SquareInternal};
use std::fmt;

/*
    FLAGS:
    0001    1: double pawn push
    0101    4: capture
    0101    5: ep capture
    1XXX     : promotion

    LAYOUT
    lower, upper
    00 01 -> double pawn push
    01 00 -> capture
    01 01 -> ep capture
    10 XX -> promotion (XX is piece to promote to)
    11 XX -> promo-capture (XX is piece to promote to)
    00 1X -> castle (X is castle type)
*/

pub type Internal = u8;
const CASTLE_FLAG: Internal = 128;
const CAPTURE_FLAG: Internal = 64;
const PROMOTION_FLAG: Internal = 128;
const EP_CAPTURE_FLAG: Internal = 64;
#[allow(dead_code)]
pub const NULL_MOVE: Move = Move { upper: 0, lower: 0 };

#[allow(dead_code)]
pub const QUEEN_SIDE_CASTLE: Move = Move {
    lower: 0,
    upper: CASTLE_FLAG | (QUEEN_SIDE.to_u8() << 6),
};

#[allow(dead_code)]
pub const KING_SIDE_CASTLE: Move = Move {
    lower: 0,
    upper: CASTLE_FLAG | (KING_SIDE.to_u8() << 6),
};

/// Represents a move on the chess position. Uses a compact 16 bit representation
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(packed(2))] // packed since often stored in transposition tables
pub struct Move {
    lower: Internal, // holds from square and castle and pawn flags
    upper: Internal, // holds to square and promotion and capture flags
}

impl Default for Move {
    fn default() -> Self {
        NULL_MOVE
    }
}

impl Move {
    pub fn from(self) -> Square {
        Square::new((self.lower & 63) as SquareInternal)
    }

    pub fn to(self) -> Square {
        Square::new((self.upper & 63) as SquareInternal)
    }

    pub fn promote_to(self) -> Kind {
        debug_assert!(!self.is_castle());
        debug_assert!(self.is_promotion());
        Kind((self.upper & (!63)) >> 6)
    }

    /// Returns the absolute distance moved. Eg for a push from square 8 to square 24: |24 - 8| = 16
    pub fn distance(self) -> i32 {
        debug_assert!(!self.is_castle());
        (self.from().to_i32() - self.to().to_i32()).abs()
    }

    pub fn is_castle(self) -> bool {
        ((self.upper & CASTLE_FLAG) != 0) && ((self.lower & (!63)) == 0)
    }

    pub fn is_capture(self) -> bool {
        (self.lower & CAPTURE_FLAG) != 0
    }

    pub fn is_ep_capture(self) -> bool {
        ((self.lower & (!63)) == CAPTURE_FLAG) && ((self.upper & (!63)) == EP_CAPTURE_FLAG)
    }

    pub fn is_promotion(self) -> bool {
        (self.lower & PROMOTION_FLAG) != 0
    }

    pub fn castle(self) -> Castle {
        debug_assert!(self.is_castle());

        Castle::new(((self.upper & 64) >> 6) as usize)
    }

    pub fn new_move(from: Square, to: Square, is_capture: bool) -> Move {
        Move {
            lower: from.to_u8() | if is_capture { CAPTURE_FLAG } else { 0 },
            upper: to.to_u8(),
        }
    }

    pub fn new_push(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8(),
            upper: to.to_u8(),
        }
    }

    pub fn new_capture(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8() | CAPTURE_FLAG,
            upper: to.to_u8(),
        }
    }

    pub fn new_castle(castle: Castle) -> Move {
        Move {
            lower: 0,
            upper: CASTLE_FLAG | (castle.to_u8() << 6),
        }
    }

    pub fn new_promotion(from: Square, to: Square, promote_to: Kind) -> Move {
        Move {
            lower: from.to_u8() | PROMOTION_FLAG,
            upper: to.to_u8() | (promote_to.to_u8() << 6),
        }
    }

    pub fn new_capture_promotion(from: Square, to: Square, promote_to: Kind) -> Move {
        Move {
            lower: from.to_u8() | PROMOTION_FLAG | CAPTURE_FLAG,
            upper: to.to_u8() | (promote_to.to_u8() << 6),
        }
    }

    pub fn new_ep_capture(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8() | CAPTURE_FLAG,
            upper: to.to_u8() | EP_CAPTURE_FLAG,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_castle() {
            return write!(f, "{}", self.castle().pgn_string());
        }

        let mut s = String::new();

        s += &self.from().to_string();

        if self.is_capture() {
            s.push('x');
        }

        s += &self.to().to_string();

        if self.is_promotion() {
            s.push('=');
            s.push(self.promote_to().to_char());
        }

        if self.is_ep_capture() {
            s += "e.p."
        }

        write!(f, "{}", &s)
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// MoveScore encodes a move and the piece-square score change
/// that the move creates
#[derive(Clone, Copy)]
pub struct MoveScore(Move, i16);

impl MoveScore {
    #[allow(dead_code)]
    pub fn mv(self) -> Move {
        self.0
    }

    #[allow(dead_code)]
    pub fn score(self) -> i16 {
        self.1
    }

    pub const fn new(mv: Move, score: i16) -> MoveScore {
        MoveScore(mv, score)
    }
}

impl fmt::Display for MoveScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}

impl fmt::Debug for MoveScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::*;
    use std::mem;

    #[test]
    fn test_packed() {
        assert_eq!(2, mem::size_of::<Move>());
    }

    #[test]
    fn push() {
        let mv = Move::new_push(B2, B3);
        assert_eq!(mv.from(), B2);
        assert_eq!(mv.to(), B3);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_castle(), false);
        assert_eq!(mv.is_promotion(), false);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn capture() {
        let mv = Move::new_capture(B2, B5);
        assert_eq!(mv.from(), B2);
        assert_eq!(mv.to(), B5);
        assert_eq!(mv.is_capture(), true);
        assert_eq!(mv.is_castle(), false);
        assert_eq!(mv.is_promotion(), false);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn promotion() {
        let mv = Move::new_promotion(B7, B8, KNIGHT);
        assert_eq!(mv.from(), B7);
        assert_eq!(mv.to(), B8);
        assert_eq!(mv.promote_to(), KNIGHT);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_castle(), false);
        assert_eq!(mv.is_promotion(), true);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn capture_promotion() {
        let mv = Move::new_capture_promotion(B7, B8, QUEEN);
        assert_eq!(mv.from(), B7);
        assert_eq!(mv.to(), B8);
        assert_eq!(mv.promote_to(), QUEEN);
        assert_eq!(mv.is_capture(), true);
        assert_eq!(mv.is_castle(), false);
        assert_eq!(mv.is_promotion(), true);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn castle_queen_side() {
        let mv = Move::new_castle(QUEEN_SIDE);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.castle(), QUEEN_SIDE);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_promotion(), false);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn castle_king_side() {
        let mv = Move::new_castle(KING_SIDE);
        assert_eq!(mv.is_castle(), true);
        assert_eq!(mv.castle(), KING_SIDE);
        assert_eq!(mv.is_capture(), false);
        assert_eq!(mv.is_promotion(), false);
        assert_eq!(mv.is_ep_capture(), false);
    }

    #[test]
    fn new_ep_capture() {
        let mv = Move::new_ep_capture(D4, C3);
        assert_eq!(mv.from(), D4);
        assert_eq!(mv.to(), C3);
        assert_eq!(mv.is_capture(), true);
        assert_eq!(mv.is_castle(), false);
        assert_eq!(mv.is_promotion(), false);
        assert_eq!(mv.is_ep_capture(), true);
    }

    #[test]
    fn to_string() {
        assert_eq!(Move::new_castle(KING_SIDE).to_string(), "O-O");
        assert_eq!(Move::new_castle(QUEEN_SIDE).to_string(), "O-O-O");

        assert_eq!(Move::new_push(B2, B3).to_string(), "b2b3");
        assert_eq!(Move::new_push(B2, D5).to_string(), "b2d5");
        assert_eq!(Move::new_capture(B2, B5).to_string(), "b2xb5");
        assert_eq!(Move::new_capture(B2, B3).to_string(), "b2xb3");

        assert_eq!(Move::new_promotion(B7, B8, QUEEN).to_string(), "b7b8=Q");
        assert_eq!(
            Move::new_capture_promotion(C7, B8, QUEEN).to_string(),
            "c7xb8=Q"
        );

        assert_eq!(Move::new_ep_capture(D4, C3).to_string(), "d4xc3e.p.");
    }
}
