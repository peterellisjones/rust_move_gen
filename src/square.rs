use bb::{ANTI_DIAGONALS, BB, BISHOP_RAYS, DIAGONALS, FILE_A, KING_MOVES, KNIGHT_MOVES, ROOK_RAYS};
use side::{Side, BLACK};
use std::fmt;

#[cfg(test)]
use rand;

pub type Internal = usize;

/// Represents a square on the chessboard
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Square(pub Internal);

const NAMES: [&'static str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

#[allow(dead_code)]
pub const A1: Square = Square(0);
#[allow(dead_code)]
pub const B1: Square = Square(1);
#[allow(dead_code)]
pub const C1: Square = Square(2);
#[allow(dead_code)]
pub const D1: Square = Square(3);
#[allow(dead_code)]
pub const E1: Square = Square(4);
#[allow(dead_code)]
pub const F1: Square = Square(5);
#[allow(dead_code)]
pub const G1: Square = Square(6);
#[allow(dead_code)]
pub const H1: Square = Square(7);
#[allow(dead_code)]
pub const A2: Square = Square(8);
#[allow(dead_code)]
pub const B2: Square = Square(9);
#[allow(dead_code)]
pub const C2: Square = Square(10);
#[allow(dead_code)]
pub const D2: Square = Square(11);
#[allow(dead_code)]
pub const E2: Square = Square(12);
#[allow(dead_code)]
pub const F2: Square = Square(13);
#[allow(dead_code)]
pub const G2: Square = Square(14);
#[allow(dead_code)]
pub const H2: Square = Square(15);
#[allow(dead_code)]
pub const A3: Square = Square(16);
#[allow(dead_code)]
pub const B3: Square = Square(17);
#[allow(dead_code)]
pub const C3: Square = Square(18);
#[allow(dead_code)]
pub const D3: Square = Square(19);
#[allow(dead_code)]
pub const E3: Square = Square(20);
#[allow(dead_code)]
pub const F3: Square = Square(21);
#[allow(dead_code)]
pub const G3: Square = Square(22);
#[allow(dead_code)]
pub const H3: Square = Square(23);
#[allow(dead_code)]
pub const A4: Square = Square(24);
#[allow(dead_code)]
pub const B4: Square = Square(25);
#[allow(dead_code)]
pub const C4: Square = Square(26);
#[allow(dead_code)]
pub const D4: Square = Square(27);
#[allow(dead_code)]
pub const E4: Square = Square(28);
#[allow(dead_code)]
pub const F4: Square = Square(29);
#[allow(dead_code)]
pub const G4: Square = Square(30);
#[allow(dead_code)]
pub const H4: Square = Square(31);
#[allow(dead_code)]
pub const A5: Square = Square(32);
#[allow(dead_code)]
pub const B5: Square = Square(33);
#[allow(dead_code)]
pub const C5: Square = Square(34);
#[allow(dead_code)]
pub const D5: Square = Square(35);
#[allow(dead_code)]
pub const E5: Square = Square(36);
#[allow(dead_code)]
pub const F5: Square = Square(37);
#[allow(dead_code)]
pub const G5: Square = Square(38);
#[allow(dead_code)]
pub const H5: Square = Square(39);
#[allow(dead_code)]
pub const A6: Square = Square(40);
#[allow(dead_code)]
pub const B6: Square = Square(41);
#[allow(dead_code)]
pub const C6: Square = Square(42);
#[allow(dead_code)]
pub const D6: Square = Square(43);
#[allow(dead_code)]
pub const E6: Square = Square(44);
#[allow(dead_code)]
pub const F6: Square = Square(45);
#[allow(dead_code)]
pub const G6: Square = Square(46);
#[allow(dead_code)]
pub const H6: Square = Square(47);
#[allow(dead_code)]
pub const A7: Square = Square(48);
#[allow(dead_code)]
pub const B7: Square = Square(49);
#[allow(dead_code)]
pub const C7: Square = Square(50);
#[allow(dead_code)]
pub const D7: Square = Square(51);
#[allow(dead_code)]
pub const E7: Square = Square(52);
#[allow(dead_code)]
pub const F7: Square = Square(53);
#[allow(dead_code)]
pub const G7: Square = Square(54);
#[allow(dead_code)]
pub const H7: Square = Square(55);
#[allow(dead_code)]
pub const A8: Square = Square(56);
#[allow(dead_code)]
pub const B8: Square = Square(57);
#[allow(dead_code)]
pub const C8: Square = Square(58);
#[allow(dead_code)]
pub const D8: Square = Square(59);
#[allow(dead_code)]
pub const E8: Square = Square(60);
#[allow(dead_code)]
pub const F8: Square = Square(61);
#[allow(dead_code)]
pub const G8: Square = Square(62);
#[allow(dead_code)]
pub const H8: Square = Square(63);

impl Square {
    pub fn new(s: Internal) -> Square {
        Square(s)
    }

    pub const fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    pub fn to_i32(&self) -> i32 {
        self.0 as i32
    }

    pub fn to_u32(&self) -> u32 {
        self.0 as u32
    }

    pub fn raw(&self) -> Internal {
        self.0
    }

    pub fn file_mask(&self) -> BB {
        FILE_A << (self.0 & 7)
    }

    pub fn same_file(&self, other: Square) -> bool {
        self.0 & 7 == other.0 & 7
    }

    pub fn file_char(&self) -> char {
        FILES[self.0 & 7]
    }

    pub fn rank_char(&self) -> char {
        RANKS[self.0 >> 3]
    }

    pub fn diagonals(&self) -> BB {
        unsafe { *DIAGONALS.get_unchecked(self.to_usize()) }
    }

    pub fn anti_diagonals(&self) -> BB {
        unsafe { *ANTI_DIAGONALS.get_unchecked(self.to_usize()) }
    }

    pub fn bishop_rays(&self) -> BB {
        unsafe { *BISHOP_RAYS.get_unchecked(self.to_usize()) }
    }

    pub fn rook_rays(&self) -> BB {
        unsafe { *ROOK_RAYS.get_unchecked(self.to_usize()) }
    }

    pub fn king_moves(&self) -> BB {
        unsafe { *KING_MOVES.get_unchecked(self.to_usize()) }
    }

    pub fn knight_moves(&self) -> BB {
        unsafe { *KNIGHT_MOVES.get_unchecked(self.to_usize()) }
    }

    pub fn inc(&mut self) {
        self.0 += 1
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    // Gives square from perspective of side
    // ie, flips if black
    pub fn from_side(&self, side: Side) -> Square {
        Square(self.0 ^ if side == BLACK { 56 } else { 0 })
    }

    pub fn flip(&self) -> Square {
        Square(self.0 ^ 56)
    }

    pub fn rotate_right(&self, amount: Internal) -> Square {
        Square((self.0 + (64 - amount)) & 63)
    }

    pub fn rotate_left(&self, amount: Internal) -> Square {
        Square((self.0 + amount) & 63)
    }

    pub fn to_str(&self) -> &'static str {
        NAMES[self.0 as usize]
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    #[cfg(test)]
    pub fn random() -> Square {
        Square(rand::random::<Internal>() % 64)
    }

    // returns a square at the same row as self, and the same col as another square
    pub fn along_row_with_col(&self, other: Square) -> Square {
        Square((self.0 & 56) | (other.0 & 7))
    }

    pub fn change_row(&self, row: Internal) -> Square {
        Square((self.0 & 7) | (row * 8))
    }

    pub fn row(&self) -> Internal {
        self.0 >> 3
    }

    pub fn rowx8(&self) -> Internal {
        self.0 & 56
    }

    pub fn col(&self) -> Internal {
        self.0 & 7
    }

    pub fn from(row: Internal, col: Internal) -> Square {
        Square(row * 8 + col)
    }

    pub fn parse(s: &str) -> Result<Option<Square>, String> {
        if s == "-" {
            return Ok(None);
        }

        if s.len() < 2 {
            return Err("String too short".to_string());
        }

        let col_char = s.chars().nth(0).unwrap() as Internal;
        let row_char = s.chars().nth(1).unwrap() as Internal;

        let col = col_char - 'a' as Internal;
        let row = row_char - '1' as Internal;

        if col > 7 {
            return Err(format!("Bad column identifier: {}", col_char));
        }

        if row > 7 {
            return Err(format!("Bad row identifier: {}", row_char));
        }

        Ok(Some(Square::from(row, col)))
    }

    pub fn iter() -> SquaresIter {
        SquaresIter(Square(0))
    }
}

#[derive(Debug)]
pub struct SquaresIter(Square);

impl Iterator for SquaresIter {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let sq = self.0;

        if sq > H8 {
            return None;
        }

        (self.0).0 += 1;

        Some(sq)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn along_row_with_col() {
        assert_eq!(A5.along_row_with_col(B6), B5);
    }

    #[test]
    fn flip() {
        assert_eq!(H8.flip(), H1);
        assert_eq!(C3.flip(), C6);
        assert_eq!(B2.flip(), B7);
    }

    #[test]
    fn rotate_left() {
        assert_eq!(A1.rotate_left(1), B1);
        assert_eq!(H8.rotate_left(1), A1);
        assert_eq!(C3.rotate_left(16), C5);
    }

    #[test]
    fn raw() {
        assert_eq!(H8.raw(), 63);
        assert_eq!(B3.raw(), 17);
    }

    #[test]
    fn to_string() {
        assert_eq!(A1.to_string(), "a1");
        assert_eq!(F8.to_string(), "f8");
    }

    #[test]
    fn col() {
        assert_eq!(A2.col(), 0);
        assert_eq!(C6.col(), 2);
    }

    #[test]
    fn row() {
        assert_eq!(A2.row(), 1);
        assert_eq!(C6.row(), 5);
    }
}
