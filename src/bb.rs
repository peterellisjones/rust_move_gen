use square;
use square::Square;
use std::fmt;
use std::ops::*;
use util::grid_to_string;

#[cfg(test)]
use rand;

/// BB represents a bitboard
#[derive(PartialEq, Copy, Clone)]
pub struct BB(pub u64);

impl BB {
    pub fn new(sq: Square) -> BB {
        BB(1u64 << sq.to_usize())
    }

    /// Creates a random bitboard with a given average density
    #[cfg(test)]
    pub fn random(density: f64) -> BB {
        let mut u = 0u64;

        for i in 0..64 {
            if rand::random::<f64>() < density {
                u |= 1u64 << i;
            }
        }

        BB(u)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    /// true if non empty
    pub fn any(&self) -> bool {
        self.0 != 0u64
    }

    pub fn none(&self) -> bool {
        self.0 == 0u64
    }

    /// swaps bytes
    pub fn bswap(&self) -> BB {
        BB(self.0.swap_bytes())
    }

    pub fn rot_left(&self, amount: u32) -> BB {
        BB(self.0.rotate_left(amount))
    }

    pub fn rot_right(&self, amount: u32) -> BB {
        BB(self.0.rotate_right(amount))
    }

    pub fn is_set(&self, sq: Square) -> bool {
        (self.0 >> sq.to_usize()) & 1 != 0
    }

    pub fn row_empty(&self, row: usize) -> bool {
        let row_mask = 0xFFu64 << (row * 8);
        (self.0 & row_mask) == 0u64
    }

    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn to_string(&self) -> String {
        let fun = |sq: Square| -> char {
            if self.is_set(sq) {
                '#'
            } else {
                '.'
            }
        };

        grid_to_string(fun)
    }

    fn lsb(&self) -> BB {
        BB(self.0 & 0u64.wrapping_sub(self.0))
    }

    pub fn bitscan(&self) -> Square {
        Square::new(self.0.trailing_zeros() as square::Internal)
    }

    pub fn msb(&self) -> u32 {
        debug_assert!(self.0 != 0);
        63 ^ self.0.leading_zeros()
    }

    pub fn leading_zeros(&self) -> u32 {
        debug_assert!(self.0 != 0);
        self.0.leading_zeros()
    }

    pub fn bitscan_reverse(&self) -> u32 {
        self.0.leading_zeros() ^ 63
    }

    pub fn iter(self) -> BBIterator {
        BBIterator(self)
    }

    pub fn square_list(&self) -> Vec<Square> {
        self.iter().map(|(sq, _)| sq).collect::<Vec<Square>>()
    }

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

    pub fn east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_east_fill(empty);

        (gen << 1) & NOT_FILE_A
    }

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

    pub fn north_east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_east_fill(empty);

        (gen << 9) & NOT_FILE_A
    }

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

    pub fn north_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_fill(empty);

        gen << 8
    }

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

    pub fn south_east_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_east_fill(empty);

        (gen >> 7) & NOT_FILE_A
    }

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

    pub fn west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_west_fill(empty);

        (gen >> 1) & NOT_FILE_H
    }

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

    pub fn south_west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_west_fill(empty);

        (gen >> 9) & NOT_FILE_H
    }

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

    pub fn north_west_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_north_west_fill(empty);

        (gen << 7) & NOT_FILE_H
    }

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

    pub fn south_attacks(&self, empty: BB) -> BB {
        let gen = self.occluded_south_fill(empty);

        gen >> 8
    }

    pub fn occluded_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_east_fill(empty);

        BB(gen.0 | ((gen.0 << 1) & NOT_FILE_A.0))
    }

    pub fn occluded_north_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_east_fill(empty);

        BB(gen.0 | ((gen.0 << 9) & NOT_FILE_A.0))
    }

    pub fn occluded_north_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_fill(empty);

        BB(gen.0 | (gen.0 << 8))
    }

    pub fn occluded_south_east_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_east_fill(empty);

        BB(gen.0 | ((gen.0 >> 7) & NOT_FILE_A.0))
    }

    pub fn occluded_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_west_fill(empty);

        BB(gen.0 | ((gen.0 >> 1) & NOT_FILE_H.0))
    }

    pub fn occluded_south_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_west_fill(empty);

        BB(gen.0 | ((gen.0 >> 9) & NOT_FILE_H.0))
    }

    pub fn occluded_north_west_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_north_west_fill(empty);

        BB(gen.0 | ((gen.0 << 7) & NOT_FILE_H.0))
    }

    pub fn occluded_south_fill_with_occluders(&self, empty: BB) -> BB {
        let gen = self.occluded_south_fill(empty);

        BB(gen.0 | (gen.0 >> 8))
    }
}

impl Shr<usize> for BB {
    type Output = BB;

    fn shr(self, amount: usize) -> BB {
        BB(self.0 >> amount)
    }
}

impl Shl<usize> for BB {
    type Output = BB;

    fn shl(self, amount: usize) -> BB {
        BB(self.0 << amount)
    }
}

impl Not for BB {
    type Output = BB;

    fn not(self) -> BB {
        BB(!self.0)
    }
}

impl BitOr for BB {
    type Output = BB;

    fn bitor(self, other: BB) -> BB {
        BB(self.0 | other.0)
    }
}

impl BitOrAssign for BB {
    fn bitor_assign(&mut self, other: BB) {
        self.0 |= other.0
    }
}

impl BitXor for BB {
    type Output = BB;

    fn bitxor(self, other: BB) -> BB {
        BB(self.0 ^ other.0)
    }
}

impl BitXorAssign for BB {
    fn bitxor_assign(&mut self, other: BB) {
        self.0 ^= other.0
    }
}

impl BitAnd for BB {
    type Output = BB;

    fn bitand(self, other: BB) -> BB {
        BB(self.0 & other.0)
    }
}

impl BitAndAssign for BB {
    fn bitand_assign(&mut self, other: BB) {
        self.0 &= other.0
    }
}

impl Sub for BB {
    type Output = BB;

    fn sub(self, other: BB) -> BB {
        BB(self.0.wrapping_sub(other.0))
    }
}

impl Add for BB {
    type Output = BB;

    fn add(self, other: BB) -> BB {
        BB(self.0.wrapping_add(other.0))
    }
}

impl Mul for BB {
    type Output = BB;

    fn mul(self, other: BB) -> BB {
        BB(self.0.wrapping_mul(other.0))
    }
}

impl Neg for BB {
    type Output = BB;

    fn neg(self) -> BB {
        BB(self.0.wrapping_neg())
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
pub const FILE_B: BB = BB(FILE_A.0 << 1);
pub const FILE_G: BB = BB(FILE_A.0 << 6);
pub const FILE_H: BB = BB(FILE_A.0 << 7);
pub const NOT_FILE_A: BB = BB(!FILE_A.0);
pub const NOT_FILE_H: BB = BB(!FILE_H.0);
pub const ROW_1: BB = BB(0xFFu64);
pub const ROW_2: BB = BB(ROW_1.0 << (8));
pub const ROW_4: BB = BB(ROW_1.0 << (3 * 8));
pub const ROW_5: BB = BB(ROW_1.0 << (4 * 8));
pub const ROW_7: BB = BB(ROW_1.0 << (6 * 8));
pub const ROW_8: BB = BB(ROW_1.0 << (7 * 8));
pub const EDGES: BB = BB(FILE_A.0 | FILE_H.0 | ROW_1.0 | ROW_8.0);

/// `BBIterator` iterates over set bits in a bitboard, from low to high,
/// returning a Square and the bit-board with that bit set
pub struct BBIterator(BB);

impl Iterator for BBIterator {
    type Item = (Square, BB);

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
    use square::*;
    use unindent;

    #[test]
    fn test_occluded_east_fill() {
        let expected = unindent::unindent(
            "
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
            ",
        );
        let source = BB::new(C3) | BB::new(D6);
        let empty = source | BB::new(F3).not();
        assert_eq!(source.occluded_east_fill(empty).to_string(), expected);
    }

    #[test]
    fn test_east_attacks() {
        let expected = unindent::unindent(
            "
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
            ",
        );
        let source = BB::new(C3) | BB::new(D6);
        let empty = source | BB::new(F3).not();
        assert_eq!(source.east_attacks(empty).to_string(), expected);
    }

    #[test]
    fn consts_1() {
        let expected = unindent::unindent(
            "
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
            ",
        );
        assert_eq!(FILE_A.to_string(), expected);
    }

    #[test]
    fn consts_2() {
        let expected = unindent::unindent(
            "
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
            ",
        );
        assert_eq!(FILE_H.to_string(), expected);
    }

    #[test]
    fn consts_4() {
        let expected = unindent::unindent(
            "
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
            ",
        );
        assert_eq!(END_ROWS.to_string(), expected);
    }
}

pub const DIAGONALS: [BB; 64] = [
    BB(0),
    BB(256),
    BB(66048),
    BB(16909312),
    BB(4328785920),
    BB(1108169199616),
    BB(283691315109888),
    BB(72624976668147712),
    BB(2),
    BB(65540),
    BB(16908296),
    BB(4328783888),
    BB(1108169195552),
    BB(283691315101760),
    BB(72624976668131456),
    BB(145249953336262656),
    BB(516),
    BB(16778248),
    BB(4328523792),
    BB(1108168675360),
    BB(283691314061376),
    BB(72624976666050688),
    BB(145249953332101120),
    BB(290499906664136704),
    BB(132104),
    BB(4295231504),
    BB(1108102090784),
    BB(283691180892224),
    BB(72624976399712384),
    BB(145249952799424512),
    BB(290499905598783488),
    BB(580999811180789760),
    BB(33818640),
    BB(1099579265056),
    BB(283674135240768),
    BB(72624942308409472),
    BB(145249884616818688),
    BB(290499769233571840),
    BB(580999538450366464),
    BB(1161999072605765632),
    BB(8657571872),
    BB(281492291854400),
    BB(72620578621636736),
    BB(145241157243273216),
    BB(290482314486480896),
    BB(580964628956184576),
    BB(1161929253617401856),
    BB(2323857407723175936),
    BB(2216338399296),
    BB(72062026714726528),
    BB(144124053429452800),
    BB(288248106858840064),
    BB(576496213700902912),
    BB(1152992423106838528),
    BB(2305983746702049280),
    BB(4611686018427387904),
    BB(567382630219904),
    BB(1134765260439552),
    BB(2269530520813568),
    BB(4539061024849920),
    BB(9078117754732544),
    BB(18155135997837312),
    BB(36028797018963968),
    BB(0),
];

pub const ANTI_DIAGONALS: [BB; 64] = [
    BB(9241421688590303744),
    BB(36099303471055872),
    BB(141012904183808),
    BB(550831656960),
    BB(2151686144),
    BB(8404992),
    BB(32768),
    BB(0),
    BB(4620710844295151616),
    BB(9241421688590303233),
    BB(36099303471054850),
    BB(141012904181764),
    BB(550831652872),
    BB(2151677968),
    BB(8388640),
    BB(64),
    BB(2310355422147510272),
    BB(4620710844295020800),
    BB(9241421688590041601),
    BB(36099303470531586),
    BB(141012903135236),
    BB(550829559816),
    BB(2147491856),
    BB(16416),
    BB(1155177711056977920),
    BB(2310355422114021376),
    BB(4620710844228043008),
    BB(9241421688456086017),
    BB(36099303202620418),
    BB(141012367312900),
    BB(549757915144),
    BB(4202512),
    BB(577588851233521664),
    BB(1155177702483820544),
    BB(2310355404967706624),
    BB(4620710809935413504),
    BB(9241421619870827009),
    BB(36099166032102402),
    BB(140738026276868),
    BB(1075843080),
    BB(288793326105133056),
    BB(577586656505233408),
    BB(1155173313027244032),
    BB(2310346626054553600),
    BB(4620693252109107456),
    BB(9241386504218214913),
    BB(36028934726878210),
    BB(275415828484),
    BB(144115188075855872),
    BB(288231475663339520),
    BB(576462955621646336),
    BB(1152925911260069888),
    BB(2305851822520205312),
    BB(4611703645040410880),
    BB(9223407290080821761),
    BB(70506452091906),
    BB(0),
    BB(281474976710656),
    BB(564049465049088),
    BB(1128103225065472),
    BB(2256206466908160),
    BB(4512412933881856),
    BB(9024825867763968),
    BB(18049651735527937),
];

pub const BOTH_DIAGONALS: [BB; 64] = [
    BB(0 | 9241421688590303744),
    BB(256 | 36099303471055872),
    BB(66048 | 141012904183808),
    BB(16909312 | 550831656960),
    BB(4328785920 | 2151686144),
    BB(1108169199616 | 8404992),
    BB(283691315109888 | 32768),
    BB(72624976668147712 | 0),
    BB(2 | 4620710844295151616),
    BB(65540 | 9241421688590303233),
    BB(16908296 | 36099303471054850),
    BB(4328783888 | 141012904181764),
    BB(1108169195552 | 550831652872),
    BB(283691315101760 | 2151677968),
    BB(72624976668131456 | 8388640),
    BB(145249953336262656 | 64),
    BB(516 | 2310355422147510272),
    BB(16778248 | 4620710844295020800),
    BB(4328523792 | 9241421688590041601),
    BB(1108168675360 | 36099303470531586),
    BB(283691314061376 | 141012903135236),
    BB(72624976666050688 | 550829559816),
    BB(145249953332101120 | 2147491856),
    BB(290499906664136704 | 16416),
    BB(132104 | 1155177711056977920),
    BB(4295231504 | 2310355422114021376),
    BB(1108102090784 | 4620710844228043008),
    BB(283691180892224 | 9241421688456086017),
    BB(72624976399712384 | 36099303202620418),
    BB(145249952799424512 | 141012367312900),
    BB(290499905598783488 | 549757915144),
    BB(580999811180789760 | 4202512),
    BB(33818640 | 577588851233521664),
    BB(1099579265056 | 1155177702483820544),
    BB(283674135240768 | 2310355404967706624),
    BB(72624942308409472 | 4620710809935413504),
    BB(145249884616818688 | 9241421619870827009),
    BB(290499769233571840 | 36099166032102402),
    BB(580999538450366464 | 140738026276868),
    BB(1161999072605765632 | 1075843080),
    BB(8657571872 | 288793326105133056),
    BB(281492291854400 | 577586656505233408),
    BB(72620578621636736 | 1155173313027244032),
    BB(145241157243273216 | 2310346626054553600),
    BB(290482314486480896 | 4620693252109107456),
    BB(580964628956184576 | 9241386504218214913),
    BB(1161929253617401856 | 36028934726878210),
    BB(2323857407723175936 | 275415828484),
    BB(2216338399296 | 144115188075855872),
    BB(72062026714726528 | 288231475663339520),
    BB(144124053429452800 | 576462955621646336),
    BB(288248106858840064 | 1152925911260069888),
    BB(576496213700902912 | 2305851822520205312),
    BB(1152992423106838528 | 4611703645040410880),
    BB(2305983746702049280 | 9223407290080821761),
    BB(4611686018427387904 | 70506452091906),
    BB(567382630219904 | 0),
    BB(1134765260439552 | 281474976710656),
    BB(2269530520813568 | 564049465049088),
    BB(4539061024849920 | 1128103225065472),
    BB(9078117754732544 | 2256206466908160),
    BB(18155135997837312 | 4512412933881856),
    BB(36028797018963968 | 9024825867763968),
    BB(0 | 18049651735527937),
];
