use castle::Castle;
use piece::*;
use square;
use square::Square;
use std::fmt;

// super compress move representation
// 0-5: from
// 6-11: to
// 12-15: flags

/*
    FLAGS:
    0001    1: double pawn push
    0101    4: capture
    0101    5: ep capture
    1XXX     : promotion

    ALT LAYOUT
    lower, upper
    00 01 -> double pawn push
    01 00 -> capture
    01 01 -> ep capture
    10 XX -> promotion (XX is piece to promote to)
    11 XX -> promo-capture (XX is piece to promote to)
    00 1X -> castle (X is castle type)
*/

const CASTLE_FLAG: u8 = 128;
const CAPTURE_FLAG: u8 = 64;
const PROMOTION_FLAG: u8 = 128;
const EP_CAPTURE_FLAG: u8 = 64;
#[allow(dead_code)]
pub const NULL_MOVE: Move = Move { upper: 0, lower: 0 };

/// Represents a move on the chess position. Uses a compact 16 bit representation
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    lower: u8, // holds from square and castle and pawn flags
    upper: u8, // holds to square and promotion and capture flags
}

impl Move {
    pub fn from(&self) -> Square {
        Square::new((self.lower & 63) as square::Internal)
    }

    pub fn to(&self) -> Square {
        Square::new((self.upper & 63) as square::Internal)
    }

    pub fn promote_to(&self) -> Kind {
        debug_assert!(!self.is_castle());
        debug_assert!(self.is_promotion());
        Kind(((self.upper as usize) & (!63)) >> 6)
    }

    /// Returns the absolute distance moved. Eg for a push from square 8 to square 24: |24 - 8| = 16
    pub fn distance(&self) -> i32 {
        debug_assert!(!self.is_castle());
        (self.from().to_i32() - self.to().to_i32()).abs()
    }

    pub fn is_castle(&self) -> bool {
        ((self.upper & CASTLE_FLAG) != 0) && ((self.lower & (!63)) == 0)
    }

    pub fn is_capture(&self) -> bool {
        (self.lower & CAPTURE_FLAG) != 0
    }

    pub fn is_ep_capture(&self) -> bool {
        ((self.lower & (!63)) == CAPTURE_FLAG) && ((self.upper & (!63)) == EP_CAPTURE_FLAG)
    }

    pub fn is_promotion(&self) -> bool {
        (self.lower & PROMOTION_FLAG) != 0
    }

    pub fn castle(&self) -> Castle {
        debug_assert!(self.is_castle());

        Castle::new(((self.upper & 64) >> 6) as usize)
    }

    pub fn to_string(&self) -> String {
        if self.is_castle() {
            return self.castle().pgn_string().to_string();
        }

        let mut s = String::new();

        s += self.from().to_str();

        if self.is_capture() {
            s.push('x');
        }

        s += self.to().to_str();

        if self.is_promotion() {
            s.push('=');
            s.push(self.promote_to().to_char());
        }

        if self.is_ep_capture() {
            s += "e.p."
        }

        s
    }

    pub fn new_move(from: Square, to: Square, is_capture: bool) -> Move {
        Move {
            lower: from.to_u8() | if is_capture { CAPTURE_FLAG } else { 0 },
            upper: to.to_u8(),
        }
    }

    pub const fn new_push(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8(),
            upper: to.to_u8(),
        }
    }

    pub const fn new_capture(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8() | CAPTURE_FLAG,
            upper: to.to_u8(),
        }
    }

    pub const fn new_castle(castle: Castle) -> Move {
        Move {
            lower: 0,
            upper: CASTLE_FLAG | (castle.to_u8() << 6),
        }
    }

    pub const fn new_promotion(from: Square, to: Square, promote_to: Kind) -> Move {
        Move {
            lower: from.to_u8() | PROMOTION_FLAG,
            upper: to.to_u8() | (promote_to.to_u8() << 6),
        }
    }

    pub const fn new_capture_promotion(from: Square, to: Square, promote_to: Kind) -> Move {
        Move {
            lower: from.to_u8() | PROMOTION_FLAG | CAPTURE_FLAG,
            upper: to.to_u8() | (promote_to.to_u8() << 6),
        }
    }

    pub const fn new_ep_capture(from: Square, to: Square) -> Move {
        Move {
            lower: from.to_u8() | CAPTURE_FLAG,
            upper: to.to_u8() | EP_CAPTURE_FLAG,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use castle::*;
    use square::*;
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
