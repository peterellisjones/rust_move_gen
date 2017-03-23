use std::fmt;
use std::num::Wrapping;
use square::*;
use std::ops::*;
use util::grid_to_string;

#[cfg(test)]
use rand;

/// BB represents a bitboard
#[derive(PartialEq, Copy, Clone)]
pub struct BB(pub u64);

impl BB {
    #[inline]
    pub fn new(sq: Square) -> BB {
        BB(1u64 << sq.to_usize())
    }

    /// Creates a random bitboard with a given average density
    #[inline]
    #[cfg(test)]
    pub fn random(density: f64) -> BB {
        let mut u = 0u64;

        for i in 0..64 {
            if rand::random::<f64>() < density {
                u &= 1u64 << i;
            }
        }

        BB(u)
    }

    #[inline]
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    /// true if non empty
    #[inline]
    pub fn any(&self) -> bool {
        self.0 != 0u64
    }

    /// swaps bytes
    #[inline]
    pub fn bswap(&self) -> BB {
        BB(self.0.swap_bytes())
    }

    #[inline]
    pub fn rot_left(&self, amount: u32) -> BB {
        BB(self.0.rotate_left(amount))
    }

    #[inline]
    pub fn rot_right(&self, amount: u32) -> BB {
        BB(self.0.rotate_right(amount))
    }

    #[inline]
    pub fn is_set(&self, sq: Square) -> bool {
        (self.0 >> sq.to_usize()) & 1 != 0
    }

    #[inline]
    pub fn row_empty(&self, row: usize) -> bool {
        let row_mask = 0xFFu64 << (row * 8);
        (self.0 & row_mask) == 0u64
    }

    #[inline]
    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn to_string(&self) -> String {
        grid_to_string(|sq: Square| -> char { if self.is_set(sq) { '#' } else { '.' } })
    }

    #[inline]
    fn lsb(&self) -> BB {
        BB(self.0 & 0u64.wrapping_sub(self.0))
    }

    #[inline]
    pub fn bitscan(&self) -> Square {
        Square::new(self.0.trailing_zeros() as usize)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn bitscan_reverse(&self) -> Square {
        Square::new((self.0.leading_zeros() ^ 63) as usize)
    }

    #[inline]
    pub fn iter(self) -> BBIterator {
        BBIterator(self)
    }

    #[inline]
    pub fn square_list(&self) -> Vec<Square> {
        self.iter().map(|(sq, _)| sq).collect::<Vec<Square>>()
    }

    #[inline]
    pub fn occluded_east_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 1);
        prop &= prop << 1;
        gen |= prop & (gen << 2);
        prop &= prop << 2;
        gen |= prop & (gen << 4);

        BB(gen)
    }

    #[inline]
    pub fn east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_east_fill(empty);

        (gen << 1) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_north_east_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen << 9);
        prop &= prop << 9;
        gen |= prop & (gen << 18);
        prop &= prop << 18;
        gen |= prop & (gen << 36);

        BB(gen)
    }

    #[inline]
    pub fn north_east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_east_fill(empty);

        (gen << 9) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_north_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen |= prop & (gen << 8);
        prop &= prop << 8;
        gen |= prop & (gen << 16);
        prop &= prop << 16;
        gen |= prop & (gen << 32);

        BB(gen)
    }

    #[inline]
    pub fn north_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_fill(empty);

        gen << 8
    }

    #[inline]
    pub fn occluded_south_east_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 7);
        prop &= prop >> 7;
        gen |= prop & (gen >> 14);
        prop &= prop >> 14;
        gen |= prop & (gen >> 28);

        BB(gen)
    }

    #[inline]
    pub fn south_east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_east_fill(empty);

        (gen >> 7) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_west_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 1);
        prop &= prop >> 1;
        gen |= prop & (gen >> 2);
        prop &= prop >> 2;
        gen |= prop & (gen >> 4);

        BB(gen)
    }

    #[inline]
    pub fn west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_west_fill(empty);

        (gen >> 1) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_south_west_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 9);
        prop &= prop >> 9;
        gen |= prop & (gen >> 18);
        prop &= prop >> 18;
        gen |= prop & (gen >> 36);

        BB(gen)
    }

    #[inline]
    pub fn south_west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_west_fill(empty);

        (gen >> 9) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_north_west_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen |= prop & (gen << 7);
        prop &= prop << 7;
        gen |= prop & (gen << 14);
        prop &= prop << 14;
        gen |= prop & (gen << 28);

        BB(gen)
    }

    #[inline]
    pub fn north_west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_west_fill(empty);

        (gen << 7) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_south_fill(&self, empty: BB) -> BB {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen |= prop & (gen >> 8);
        prop &= prop >> 8;
        gen |= prop & (gen >> 16);
        prop &= prop >> 16;
        gen |= prop & (gen >> 32);

        BB(gen)
    }

    #[inline]
    pub fn south_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_fill(empty);

        gen >> 8
    }

    #[inline]
    pub fn occluded_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_east_fill(empty);

        BB(gen.0 | ((gen.0 << 1) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_north_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_east_fill(empty);

        BB(gen.0 | ((gen.0 << 9) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_north_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_fill(empty);

        BB(gen.0 | (gen.0 << 8))
    }

    #[inline]
    pub fn occluded_south_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_east_fill(empty);

        BB(gen.0 | ((gen.0 >> 7) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_west_fill(empty);

        BB(gen.0 | ((gen.0 >> 1) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_south_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_west_fill(empty);

        BB(gen.0 | ((gen.0 >> 9) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_north_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_west_fill(empty);

        BB(gen.0 | ((gen.0 << 7) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_south_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_fill(empty);

        BB(gen.0 | (gen.0 >> 8))
    }
}


impl Shr<usize> for BB {
    type Output = BB;

    #[inline]
    fn shr(self, amount: usize) -> BB {
        BB(self.0 >> amount)
    }
}

impl Shl<usize> for BB {
    type Output = BB;

    #[inline]
    fn shl(self, amount: usize) -> BB {
        BB(self.0 << amount)
    }
}

impl Not for BB {
    type Output = BB;

    #[inline]
    fn not(self) -> BB {
        BB(!self.0)
    }
}

impl BitOr for BB {
    type Output = BB;

    #[inline]
    fn bitor(self, other: BB) -> BB {
        BB(self.0 | other.0)
    }
}

impl BitOrAssign for BB {
    #[inline]
    fn bitor_assign(&mut self, other: BB) {
        self.0 |= other.0
    }
}


impl BitXor for BB {
    type Output = BB;

    #[inline]
    fn bitxor(self, other: BB) -> BB {
        BB(self.0 ^ other.0)
    }
}

impl BitXorAssign for BB {
    #[inline]
    fn bitxor_assign(&mut self, other: BB) {
        self.0 ^= other.0
    }
}

impl BitAnd for BB {
    type Output = BB;

    #[inline]
    fn bitand(self, other: BB) -> BB {
        BB(self.0 & other.0)
    }
}

impl BitAndAssign for BB {
    #[inline]
    fn bitand_assign(&mut self, other: BB) {
        self.0 &= other.0
    }
}

impl Sub for BB {
    type Output = BB;

    #[inline]
    fn sub(self, other: BB) -> BB {
        BB((Wrapping(self.0) - Wrapping(other.0)).0)
    }
}


#[allow(dead_code)]
pub const BB_A1: BB = BB(1u64 << 1);
#[allow(dead_code)]
pub const BB_B1: BB = BB(1u64 << 2);
#[allow(dead_code)]
pub const BB_C1: BB = BB(1u64 << 3);
#[allow(dead_code)]
pub const BB_D1: BB = BB(1u64 << 4);
#[allow(dead_code)]
pub const BB_C6: BB = BB(1u64 << 42);
#[allow(dead_code)]
pub const BB_D3: BB = BB(1u64 << 19);
#[allow(dead_code)]
pub const BB_H6: BB = BB(1u64 << 47);

pub const EMPTY: BB = BB(0);
pub const END_ROWS: BB = BB(ROW_1.0 | ROW_8.0);
pub const FILE_A: BB = BB(0x0101010101010101u64);
pub const FILE_H: BB = BB(0x0101010101010101u64 << 7);
pub const NOT_FILE_A: BB = BB(!FILE_A.0);
pub const NOT_FILE_H: BB = BB(!FILE_H.0);
pub const ROW_1: BB = BB(0xFFu64);
pub const ROW_4: BB = BB(0xFFu64 << (3 * 8));
pub const ROW_5: BB = BB(0xFFu64 << (4 * 8));
pub const ROW_8: BB = BB(0xFFu64 << (7 * 8));

/// `BBIterator` iterates over set bits in a bitboard, from low to high, returning a Square and the bit-board with that bit set
pub struct BBIterator(BB);

impl Iterator for BBIterator {
    type Item = (Square, BB);

    #[inline]
    fn next(&mut self) -> Option<(Square, BB)> {
        if (self.0).0 == EMPTY.0 {
            return None;
        }

        let sq = self.0.bitscan();
        let lsb = self.0.lsb();
        self.0 ^= lsb;
        Some((sq, lsb))
    }
}

impl fmt::Display for BB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for BB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
pub mod test {
    use super::*;
    use unindent;

    #[test]
    fn test_occluded_east_fill() {
        let expected = unindent::unindent("
              ABCDEFGH
            8|........|8
            7|........|7
            6|...#####|6
            5|........|5
            4|........|4
            3|..###...|3
            2|........|2
            1|........|1
              ABCDEFGH
            ");
        let source = BB::new(C3) | BB::new(D6);
        let empty = source | BB::new(F3).not();
        assert_eq!(source.occluded_east_fill(empty).to_string(), expected);
    }

    #[test]
    fn test_east_attacks() {
        let expected = unindent::unindent("
              ABCDEFGH
            8|........|8
            7|........|7
            6|....####|6
            5|........|5
            4|........|4
            3|...###..|3
            2|........|2
            1|........|1
              ABCDEFGH
            ");
        let source = BB::new(C3) | BB::new(D6);
        let empty = source | BB::new(F3).not();
        assert_eq!(source.east_attacks(empty).to_string(), expected);
    }

    #[test]
    fn consts_1() {
        let expected = unindent::unindent("
              ABCDEFGH
            8|#.......|8
            7|#.......|7
            6|#.......|6
            5|#.......|5
            4|#.......|4
            3|#.......|3
            2|#.......|2
            1|#.......|1
              ABCDEFGH
            ");
        assert_eq!(FILE_A.to_string(), expected);
    }

    #[test]
    fn consts_2() {
        let expected = unindent::unindent("
              ABCDEFGH
            8|.......#|8
            7|.......#|7
            6|.......#|6
            5|.......#|5
            4|.......#|4
            3|.......#|3
            2|.......#|2
            1|.......#|1
              ABCDEFGH
            ");
        assert_eq!(FILE_H.to_string(), expected);
    }


    #[test]
    fn consts_4() {
        let expected = unindent::unindent("
              ABCDEFGH
            8|########|8
            7|........|7
            6|........|6
            5|........|5
            4|........|4
            3|........|3
            2|........|2
            1|########|1
              ABCDEFGH
            ");
        assert_eq!(END_ROWS.to_string(), expected);
    }
}
