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

pub const BISHOP_RAYS: [BB; 64] = [
  BB(9241421688590303744),
  BB(36099303471056128),
  BB(141012904249856),
  BB(550848566272),
  BB(6480472064),
  BB(1108177604608),
  BB(283691315142656),
  BB(72624976668147712),
  BB(4620710844295151618),
  BB(9241421688590368773),
  BB(36099303487963146),
  BB(141017232965652),
  BB(1659000848424),
  BB(283693466779728),
  BB(72624976676520096),
  BB(145249953336262720),
  BB(2310355422147510788),
  BB(4620710844311799048),
  BB(9241421692918565393),
  BB(36100411639206946),
  BB(424704217196612),
  BB(72625527495610504),
  BB(145249955479592976),
  BB(290499906664153120),
  BB(1155177711057110024),
  BB(2310355426409252880),
  BB(4620711952330133792),
  BB(9241705379636978241),
  BB(108724279602332802),
  BB(145390965166737412),
  BB(290500455356698632),
  BB(580999811184992272),
  BB(577588851267340304),
  BB(1155178802063085600),
  BB(2310639079102947392),
  BB(4693335752243822976),
  BB(9386671504487645697),
  BB(326598935265674242),
  BB(581140276476643332),
  BB(1161999073681608712),
  BB(288793334762704928),
  BB(577868148797087808),
  BB(1227793891648880768),
  BB(2455587783297826816),
  BB(4911175566595588352),
  BB(9822351133174399489),
  BB(1197958188344280066),
  BB(2323857683139004420),
  BB(144117404414255168),
  BB(360293502378066048),
  BB(720587009051099136),
  BB(1441174018118909952),
  BB(2882348036221108224),
  BB(5764696068147249408),
  BB(11529391036782871041),
  BB(4611756524879479810),
  BB(567382630219904),
  BB(1416240237150208),
  BB(2833579985862656),
  BB(5667164249915392),
  BB(11334324221640704),
  BB(22667548931719168),
  BB(45053622886727936),
  BB(18049651735527937),
];

pub const ROOK_RAYS: [BB; 64] = [
  BB(72340172838076926),
  BB(144680345676153597),
  BB(289360691352306939),
  BB(578721382704613623),
  BB(1157442765409226991),
  BB(2314885530818453727),
  BB(4629771061636907199),
  BB(9259542123273814143),
  BB(72340172838141441),
  BB(144680345676217602),
  BB(289360691352369924),
  BB(578721382704674568),
  BB(1157442765409283856),
  BB(2314885530818502432),
  BB(4629771061636939584),
  BB(9259542123273813888),
  BB(72340172854657281),
  BB(144680345692602882),
  BB(289360691368494084),
  BB(578721382720276488),
  BB(1157442765423841296),
  BB(2314885530830970912),
  BB(4629771061645230144),
  BB(9259542123273748608),
  BB(72340177082712321),
  BB(144680349887234562),
  BB(289360695496279044),
  BB(578721386714368008),
  BB(1157442769150545936),
  BB(2314885534022901792),
  BB(4629771063767613504),
  BB(9259542123257036928),
  BB(72341259464802561),
  BB(144681423712944642),
  BB(289361752209228804),
  BB(578722409201797128),
  BB(1157443723186933776),
  BB(2314886351157207072),
  BB(4629771607097753664),
  BB(9259542118978846848),
  BB(72618349279904001),
  BB(144956323094725122),
  BB(289632270724367364),
  BB(578984165983651848),
  BB(1157687956502220816),
  BB(2315095537539358752),
  BB(4629910699613634624),
  BB(9259541023762186368),
  BB(143553341945872641),
  BB(215330564830528002),
  BB(358885010599838724),
  BB(645993902138460168),
  BB(1220211685215703056),
  BB(2368647251370188832),
  BB(4665518383679160384),
  BB(9259260648297103488),
  BB(18302911464433844481),
  BB(18231136449196065282),
  BB(18087586418720506884),
  BB(17800486357769390088),
  BB(17226286235867156496),
  BB(16077885992062689312),
  BB(13781085504453754944),
  BB(9187484529235886208),
];


pub const KING_MOVES: [BB; 64] = [
    BB(0x0000000000000302u64),
    BB(0x0000000000000705u64),
    BB(0x0000000000000E0Au64),
    BB(0x0000000000001C14u64),
    BB(0x0000000000003828u64),
    BB(0x0000000000007050u64),
    BB(0x000000000000E0A0u64),
    BB(0x000000000000C040u64),
    BB(0x0000000000030203u64),
    BB(0x0000000000070507u64),
    BB(0x00000000000E0A0Eu64),
    BB(0x00000000001C141Cu64),
    BB(0x0000000000382838u64),
    BB(0x0000000000705070u64),
    BB(0x0000000000E0A0E0u64),
    BB(0x0000000000C040C0u64),
    BB(0x0000000003020300u64),
    BB(0x0000000007050700u64),
    BB(0x000000000E0A0E00u64),
    BB(0x000000001C141C00u64),
    BB(0x0000000038283800u64),
    BB(0x0000000070507000u64),
    BB(0x00000000E0A0E000u64),
    BB(0x00000000C040C000u64),
    BB(0x0000000302030000u64),
    BB(0x0000000705070000u64),
    BB(0x0000000E0A0E0000u64),
    BB(0x0000001C141C0000u64),
    BB(0x0000003828380000u64),
    BB(0x0000007050700000u64),
    BB(0x000000E0A0E00000u64),
    BB(0x000000C040C00000u64),
    BB(0x0000030203000000u64),
    BB(0x0000070507000000u64),
    BB(0x00000E0A0E000000u64),
    BB(0x00001C141C000000u64),
    BB(0x0000382838000000u64),
    BB(0x0000705070000000u64),
    BB(0x0000E0A0E0000000u64),
    BB(0x0000C040C0000000u64),
    BB(0x0003020300000000u64),
    BB(0x0007050700000000u64),
    BB(0x000E0A0E00000000u64),
    BB(0x001C141C00000000u64),
    BB(0x0038283800000000u64),
    BB(0x0070507000000000u64),
    BB(0x00E0A0E000000000u64),
    BB(0x00C040C000000000u64),
    BB(0x0302030000000000u64),
    BB(0x0705070000000000u64),
    BB(0x0E0A0E0000000000u64),
    BB(0x1C141C0000000000u64),
    BB(0x3828380000000000u64),
    BB(0x7050700000000000u64),
    BB(0xE0A0E00000000000u64),
    BB(0xC040C00000000000u64),
    BB(0x0203000000000000u64),
    BB(0x0507000000000000u64),
    BB(0x0A0E000000000000u64),
    BB(0x141C000000000000u64),
    BB(0x2838000000000000u64),
    BB(0x5070000000000000u64),
    BB(0xA0E0000000000000u64),
    BB(0x40C0000000000000u64),
];

pub const KNIGHT_MOVES: [BB; 64] = [
    BB(0x0000000000020400u64),
    BB(0x0000000000050800u64),
    BB(0x00000000000A1100u64),
    BB(0x0000000000142200u64),
    BB(0x0000000000284400u64),
    BB(0x0000000000508800u64),
    BB(0x0000000000A01000u64),
    BB(0x0000000000402000u64),
    BB(0x0000000002040004u64),
    BB(0x0000000005080008u64),
    BB(0x000000000A110011u64),
    BB(0x0000000014220022u64),
    BB(0x0000000028440044u64),
    BB(0x0000000050880088u64),
    BB(0x00000000A0100010u64),
    BB(0x0000000040200020u64),
    BB(0x0000000204000402u64),
    BB(0x0000000508000805u64),
    BB(0x0000000A1100110Au64),
    BB(0x0000001422002214u64),
    BB(0x0000002844004428u64),
    BB(0x0000005088008850u64),
    BB(0x000000A0100010A0u64),
    BB(0x0000004020002040u64),
    BB(0x0000020400040200u64),
    BB(0x0000050800080500u64),
    BB(0x00000A1100110A00u64),
    BB(0x0000142200221400u64),
    BB(0x0000284400442800u64),
    BB(0x0000508800885000u64),
    BB(0x0000A0100010A000u64),
    BB(0x0000402000204000u64),
    BB(0x0002040004020000u64),
    BB(0x0005080008050000u64),
    BB(0x000A1100110A0000u64),
    BB(0x0014220022140000u64),
    BB(0x0028440044280000u64),
    BB(0x0050880088500000u64),
    BB(0x00A0100010A00000u64),
    BB(0x0040200020400000u64),
    BB(0x0204000402000000u64),
    BB(0x0508000805000000u64),
    BB(0x0A1100110A000000u64),
    BB(0x1422002214000000u64),
    BB(0x2844004428000000u64),
    BB(0x5088008850000000u64),
    BB(0xA0100010A0000000u64),
    BB(0x4020002040000000u64),
    BB(0x0400040200000000u64),
    BB(0x0800080500000000u64),
    BB(0x1100110A00000000u64),
    BB(0x2200221400000000u64),
    BB(0x4400442800000000u64),
    BB(0x8800885000000000u64),
    BB(0x100010A000000000u64),
    BB(0x2000204000000000u64),
    BB(0x0004020000000000u64),
    BB(0x0008050000000000u64),
    BB(0x00110A0000000000u64),
    BB(0x0022140000000000u64),
    BB(0x0044280000000000u64),
    BB(0x0088500000000000u64),
    BB(0x0010A00000000000u64),
    BB(0x0020400000000000u64),
];
