use std::fmt;
use side::Side;

pub type Internal = usize;

/// Represents a piece for a particular side (eg black knight)
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Piece(Internal);

const CHARS: [char; 12] = ['B', 'b', 'Q', 'q', 'R', 'r', 'N', 'n', 'P', 'p', 'K', 'k'];
// const SYMBOLS: [char; 14] = ['♙', '♟', '♘', '♞', '♗', '♝', '♖', '♜',
//                             '♕', '♛', '♔', '♚', '.'];
const NAMES: [&'static str; 12] = ["white bishop",
                                   "black bishop",
                                   "white queen",
                                   "black queen",
                                   "white rook",
                                   "black rook",
                                   "white knight",
                                   "black knight",
                                   "white pawn",
                                   "black pawn",
                                   "white king",
                                   "black king"];

/// Represents a kind of piece (eg knight)
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Kind(pub Internal);

impl Kind {
    #[inline]
    pub fn pc(&self, side: Side) -> Piece {
        Piece((self.0 << 1) | side.raw() as Internal)
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    pub fn to_char(&self) -> char {
        CHARS[self.to_usize() << 1]
    }
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}


#[allow(dead_code)]
pub const BISHOP: Kind = Kind(0);
#[allow(dead_code)]
pub const QUEEN: Kind = Kind(1);
#[allow(dead_code)]
pub const ROOK: Kind = Kind(2);
#[allow(dead_code)]
pub const KNIGHT: Kind = Kind(3);
#[allow(dead_code)]
pub const PAWN: Kind = Kind(4);
#[allow(dead_code)]
pub const KING: Kind = Kind(5);

#[allow(dead_code)]
pub const WHITE_BISHOP: Piece = Piece(0);
#[allow(dead_code)]
pub const BLACK_BISHOP: Piece = Piece(1);

#[allow(dead_code)]
pub const WHITE_QUEEN: Piece = Piece(2);
#[allow(dead_code)]
pub const BLACK_QUEEN: Piece = Piece(3);

#[allow(dead_code)]
pub const WHITE_ROOK: Piece = Piece(4);
#[allow(dead_code)]
pub const BLACK_ROOK: Piece = Piece(5);


#[allow(dead_code)]
pub const WHITE_KNIGHT: Piece = Piece(6);
#[allow(dead_code)]
pub const BLACK_KNIGHT: Piece = Piece(7);

#[allow(dead_code)]
pub const WHITE_PAWN: Piece = Piece(8);
#[allow(dead_code)]
pub const BLACK_PAWN: Piece = Piece(9);

#[allow(dead_code)]
pub const WHITE_KING: Piece = Piece(10);
#[allow(dead_code)]
pub const BLACK_KING: Piece = Piece(11);

impl Piece {
    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_char(&self) -> char {
        CHARS[self.to_usize()]
    }

    #[inline]
    pub fn kind(&self) -> Kind {
        Kind(self.0 >> 1)
    }

    // assumes piece present
    #[inline]
    pub fn is_slider(&self) -> bool {
        self.0 <= BLACK_ROOK.0
    }

    pub fn to_string(&self) -> &'static str {
        NAMES[self.to_usize()]
    }

    pub fn string_plural(&self) -> String {
        NAMES[self.to_usize()].to_string() + &"s"
    }

    #[inline]
    pub fn side(&self) -> Side {
        Side(self.to_usize() & 1)
    }

    pub fn iter() -> PiecesIter {
        PiecesIter(Piece(0))
    }

    pub fn parse(chr: char) -> Result<Piece, String> {
        for pc in Piece::iter() {
            if pc.to_char() == chr {
                return Ok(pc);
            }
        }
        Err(format!("Invalid piece: {}", chr))
    }
}

#[derive(Debug)]
pub struct PiecesIter(Piece);

impl Iterator for PiecesIter {
    type Item = Piece;

    fn next(&mut self) -> Option<Piece> {
        let pc = self.0;

        if pc >= Piece(12) {
            return None;
        }

        (self.0).0 += 1;

        Some(pc)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use side::{BLACK, WHITE};

    #[test]
    fn to_char() {
        assert_eq!(BLACK_ROOK.to_char(), 'r');
        assert_eq!(WHITE_KING.to_char(), 'K');
    }

    #[test]
    fn side() {
        assert_eq!(WHITE_PAWN.side(), WHITE);
        assert_eq!(BLACK_KNIGHT.side(), BLACK);
    }

    #[test]
    fn to_usize() {
        assert_eq!(BLACK_ROOK.to_usize(), 5);
        assert_eq!(WHITE_KING.to_usize(), 10);
    }

    #[test]
    fn kind() {
        assert_eq!(WHITE_PAWN.kind(), PAWN);
        assert_eq!(BLACK_KING.kind(), KING);
    }
}
