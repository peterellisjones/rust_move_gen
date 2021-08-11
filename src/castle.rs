use side::Side;
use square::*;
use std::fmt;

/// Represents a castle move
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Castle(pub usize);

pub const QUEEN_SIDE: Castle = Castle(0);
pub const KING_SIDE: Castle = Castle(1);

const CASTLE_KING_MOVES: [[(Square, Square); 2]; 2] = [[(E1, C1), (E8, C8)], [(E1, G1), (E8, G8)]];
const CASTLE_ROOK_MOVES: [[(Square, Square); 2]; 2] = [[(A1, D1), (A8, D8)], [(H1, F1), (H8, F8)]];

pub fn castle_king_squares(side: Side, castle: Castle) -> (Square, Square) {
    CASTLE_KING_MOVES[castle.to_usize()][side.to_usize()]
}

pub fn castle_rook_squares(side: Side, castle: Castle) -> (Square, Square) {
    CASTLE_ROOK_MOVES[castle.to_usize()][side.to_usize()]
}

impl Castle {
    pub fn new(s: usize) -> Castle {
        Castle(s)
    }

    pub const fn to_usize(&self) -> usize {
        self.0
    }

    pub const fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    /// Convert to PGN string eg "O-O-O" or "O-O"
    pub fn pgn_string(&self) -> &'static str {
        if *self == QUEEN_SIDE {
            "O-O-O"
        } else {
            "O-O"
        }
    }

    pub fn iter() -> CastlesIter {
        CastlesIter(QUEEN_SIDE)
    }
}

#[derive(Debug)]
pub struct CastlesIter(Castle);

impl Iterator for CastlesIter {
    type Item = Castle;

    fn next(&mut self) -> Option<Castle> {
        let castle = self.0;

        if castle > KING_SIDE {
            return None;
        }

        (self.0).0 += 1;

        Some(castle)
    }
}

impl fmt::Display for Castle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pgn_string())
    }
}

impl fmt::Debug for Castle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pgn_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use side::*;

    #[test]
    fn test_castle_squares() {
        let qs_king_squares = castle_king_squares(WHITE, QUEEN_SIDE);
        assert_eq!(qs_king_squares, (E1, C1));
        let qs_rook_squares = castle_rook_squares(WHITE, QUEEN_SIDE);
        assert_eq!(qs_rook_squares, (A1, D1));
    }
}
