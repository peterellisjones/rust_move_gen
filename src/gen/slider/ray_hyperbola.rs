// Hyperbola quientescence sliding piece attacks
// efficient for single sliders

use bb::*;
#[cfg(target_feature = "sse3")]
use dbb::*;
use square::Square;

#[cfg(target_feature = "sse3")]
extern crate simd;

#[cfg(target_feature = "sse3")]
use self::simd::x86::sse2::*;

#[inline]
#[allow(dead_code)]
pub fn rook_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= file_attacks_from_sq(sq, occupied) | rank_attacks_from_sq(sq, occupied);
    }
    attacks
}

#[inline]
#[allow(dead_code)]
pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    file_attacks_from_sq(from, occupied) | rank_attacks_from_sq(from, occupied)
}

#[inline]
pub fn file_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let source = BB::new(from);

    let filemask = (FILE_A << from.col() as usize) & !source;
    let forward = filemask & occupied;
    let backward = forward.bswap();
    ((forward - source) ^ (backward - source.bswap()).bswap()) & filemask
}

#[inline]
pub fn rank_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let rowx8 = from.rowx8() as usize;
    let col = from.col() as usize;

    let occupancy = (occupied >> (rowx8 + 1)).to_usize() & 63;
    let attacks = unsafe { *RANK_ATTACKS.get_unchecked(occupancy).get_unchecked(col) };
    BB((attacks as u64) << rowx8)
}

#[inline]
#[allow(dead_code)]
pub fn bishop_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= bishop_attacks_from_sq(sq, occupied);
    }
    attacks
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let source = BB::new(from);

    let diag_mask = diagonals_from_sq(from);
    let forward = diag_mask & occupied;
    let backward = forward.bswap();
    let diag_attacks = ((forward - source) ^ (backward - source.bswap()).bswap()) & diag_mask;

    let anti_diag_mask = anti_diagonals_from_sq(from);
    let forward = anti_diag_mask & occupied;
    let backward = forward.bswap();
    let anti_diag_attacks =
        ((forward - source) ^ (backward - source.bswap()).bswap()) & anti_diag_mask;

    (diag_attacks | anti_diag_attacks)
}

#[cfg(target_feature = "sse3")]
#[inline]
pub fn bishop_attacks_from_sq(from: Square, occupied_bb: BB) -> BB {
    let source = DBB::splat(BB(1)) << from.to_usize();
    let source_rev = source.bswap();
    let occupied = DBB::splat(occupied_bb);

    unsafe {
        let masks = *BOTH_DIAGONALS.get_unchecked(from.to_usize());
        let forward = masks & occupied;

        let backward = forward.bswap();

        let forward_targets = forward - source;
        let backward_targets = (backward - source_rev).bswap();

        let ret = (forward_targets ^ backward_targets) & masks;

        let (high, low) = ret.extract();
        return high | low;
    }
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn diagonals_from_sq(sq: Square) -> BB {
    unsafe {
        return *DIAGONALS.get_unchecked(sq.to_usize());
    }
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn anti_diagonals_from_sq(sq: Square) -> BB {
    unsafe {
        return *ANTI_DIAGONALS.get_unchecked(sq.to_usize());
    }
}

#[cfg(test)]
mod test {
    use super::super::testing::*;
    use super::*;
    use test;

    #[test]
    fn t_rook_attacks() {
        test_rook_attacks_from_sq(rook_attacks_from_sq);
    }

    #[test]
    fn t_bishop_attacks() {
        test_bishop_attacks_from_sq(bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, rook_attacks);
    }

    #[bench]
    fn bench_bishop_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, bishop_attacks);
    }
}

/// Indexed by source square and 6-bit occupancy. 8 * 64 = 512 bytes
const RANK_ATTACKS: [[u8; 8]; 64] = [
    [254, 253, 251, 247, 239, 223, 191, 127],
    [2, 253, 250, 246, 238, 222, 190, 126],
    [6, 5, 251, 244, 236, 220, 188, 124],
    [2, 5, 250, 244, 236, 220, 188, 124],
    [14, 13, 11, 247, 232, 216, 184, 120],
    [2, 13, 10, 246, 232, 216, 184, 120],
    [6, 5, 11, 244, 232, 216, 184, 120],
    [2, 5, 10, 244, 232, 216, 184, 120],
    [30, 29, 27, 23, 239, 208, 176, 112],
    [2, 29, 26, 22, 238, 208, 176, 112],
    [6, 5, 27, 20, 236, 208, 176, 112],
    [2, 5, 26, 20, 236, 208, 176, 112],
    [14, 13, 11, 23, 232, 208, 176, 112],
    [2, 13, 10, 22, 232, 208, 176, 112],
    [6, 5, 11, 20, 232, 208, 176, 112],
    [2, 5, 10, 20, 232, 208, 176, 112],
    [62, 61, 59, 55, 47, 223, 160, 96],
    [2, 61, 58, 54, 46, 222, 160, 96],
    [6, 5, 59, 52, 44, 220, 160, 96],
    [2, 5, 58, 52, 44, 220, 160, 96],
    [14, 13, 11, 55, 40, 216, 160, 96],
    [2, 13, 10, 54, 40, 216, 160, 96],
    [6, 5, 11, 52, 40, 216, 160, 96],
    [2, 5, 10, 52, 40, 216, 160, 96],
    [30, 29, 27, 23, 47, 208, 160, 96],
    [2, 29, 26, 22, 46, 208, 160, 96],
    [6, 5, 27, 20, 44, 208, 160, 96],
    [2, 5, 26, 20, 44, 208, 160, 96],
    [14, 13, 11, 23, 40, 208, 160, 96],
    [2, 13, 10, 22, 40, 208, 160, 96],
    [6, 5, 11, 20, 40, 208, 160, 96],
    [2, 5, 10, 20, 40, 208, 160, 96],
    [126, 125, 123, 119, 111, 95, 191, 64],
    [2, 125, 122, 118, 110, 94, 190, 64],
    [6, 5, 123, 116, 108, 92, 188, 64],
    [2, 5, 122, 116, 108, 92, 188, 64],
    [14, 13, 11, 119, 104, 88, 184, 64],
    [2, 13, 10, 118, 104, 88, 184, 64],
    [6, 5, 11, 116, 104, 88, 184, 64],
    [2, 5, 10, 116, 104, 88, 184, 64],
    [30, 29, 27, 23, 111, 80, 176, 64],
    [2, 29, 26, 22, 110, 80, 176, 64],
    [6, 5, 27, 20, 108, 80, 176, 64],
    [2, 5, 26, 20, 108, 80, 176, 64],
    [14, 13, 11, 23, 104, 80, 176, 64],
    [2, 13, 10, 22, 104, 80, 176, 64],
    [6, 5, 11, 20, 104, 80, 176, 64],
    [2, 5, 10, 20, 104, 80, 176, 64],
    [62, 61, 59, 55, 47, 95, 160, 64],
    [2, 61, 58, 54, 46, 94, 160, 64],
    [6, 5, 59, 52, 44, 92, 160, 64],
    [2, 5, 58, 52, 44, 92, 160, 64],
    [14, 13, 11, 55, 40, 88, 160, 64],
    [2, 13, 10, 54, 40, 88, 160, 64],
    [6, 5, 11, 52, 40, 88, 160, 64],
    [2, 5, 10, 52, 40, 88, 160, 64],
    [30, 29, 27, 23, 47, 80, 160, 64],
    [2, 29, 26, 22, 46, 80, 160, 64],
    [6, 5, 27, 20, 44, 80, 160, 64],
    [2, 5, 26, 20, 44, 80, 160, 64],
    [14, 13, 11, 23, 40, 80, 160, 64],
    [2, 13, 10, 22, 40, 80, 160, 64],
    [6, 5, 11, 20, 40, 80, 160, 64],
    [2, 5, 10, 20, 40, 80, 160, 64],
];

#[cfg(not(target_feature = "sse3"))]
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

#[cfg(not(target_feature = "sse3"))]
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

// 8 * 64 = 512 bytes
#[cfg(target_feature = "sse3")]
pub const BOTH_DIAGONALS: [DBB; 64] = [
    DBB(u64x2::new(0, 9241421688590303744)),
    DBB(u64x2::new(256, 36099303471055872)),
    DBB(u64x2::new(66048, 141012904183808)),
    DBB(u64x2::new(16909312, 550831656960)),
    DBB(u64x2::new(4328785920, 2151686144)),
    DBB(u64x2::new(1108169199616, 8404992)),
    DBB(u64x2::new(283691315109888, 32768)),
    DBB(u64x2::new(72624976668147712, 0)),
    DBB(u64x2::new(2, 4620710844295151616)),
    DBB(u64x2::new(65540, 9241421688590303233)),
    DBB(u64x2::new(16908296, 36099303471054850)),
    DBB(u64x2::new(4328783888, 141012904181764)),
    DBB(u64x2::new(1108169195552, 550831652872)),
    DBB(u64x2::new(283691315101760, 2151677968)),
    DBB(u64x2::new(72624976668131456, 8388640)),
    DBB(u64x2::new(145249953336262656, 64)),
    DBB(u64x2::new(516, 2310355422147510272)),
    DBB(u64x2::new(16778248, 4620710844295020800)),
    DBB(u64x2::new(4328523792, 9241421688590041601)),
    DBB(u64x2::new(1108168675360, 36099303470531586)),
    DBB(u64x2::new(283691314061376, 141012903135236)),
    DBB(u64x2::new(72624976666050688, 550829559816)),
    DBB(u64x2::new(145249953332101120, 2147491856)),
    DBB(u64x2::new(290499906664136704, 16416)),
    DBB(u64x2::new(132104, 1155177711056977920)),
    DBB(u64x2::new(4295231504, 2310355422114021376)),
    DBB(u64x2::new(1108102090784, 4620710844228043008)),
    DBB(u64x2::new(283691180892224, 9241421688456086017)),
    DBB(u64x2::new(72624976399712384, 36099303202620418)),
    DBB(u64x2::new(145249952799424512, 141012367312900)),
    DBB(u64x2::new(290499905598783488, 549757915144)),
    DBB(u64x2::new(580999811180789760, 4202512)),
    DBB(u64x2::new(33818640, 577588851233521664)),
    DBB(u64x2::new(1099579265056, 1155177702483820544)),
    DBB(u64x2::new(283674135240768, 2310355404967706624)),
    DBB(u64x2::new(72624942308409472, 4620710809935413504)),
    DBB(u64x2::new(145249884616818688, 9241421619870827009)),
    DBB(u64x2::new(290499769233571840, 36099166032102402)),
    DBB(u64x2::new(580999538450366464, 140738026276868)),
    DBB(u64x2::new(1161999072605765632, 1075843080)),
    DBB(u64x2::new(8657571872, 288793326105133056)),
    DBB(u64x2::new(281492291854400, 577586656505233408)),
    DBB(u64x2::new(72620578621636736, 1155173313027244032)),
    DBB(u64x2::new(145241157243273216, 2310346626054553600)),
    DBB(u64x2::new(290482314486480896, 4620693252109107456)),
    DBB(u64x2::new(580964628956184576, 9241386504218214913)),
    DBB(u64x2::new(1161929253617401856, 36028934726878210)),
    DBB(u64x2::new(2323857407723175936, 275415828484)),
    DBB(u64x2::new(2216338399296, 144115188075855872)),
    DBB(u64x2::new(72062026714726528, 288231475663339520)),
    DBB(u64x2::new(144124053429452800, 576462955621646336)),
    DBB(u64x2::new(288248106858840064, 1152925911260069888)),
    DBB(u64x2::new(576496213700902912, 2305851822520205312)),
    DBB(u64x2::new(1152992423106838528, 4611703645040410880)),
    DBB(u64x2::new(2305983746702049280, 9223407290080821761)),
    DBB(u64x2::new(4611686018427387904, 70506452091906)),
    DBB(u64x2::new(567382630219904, 0)),
    DBB(u64x2::new(1134765260439552, 281474976710656)),
    DBB(u64x2::new(2269530520813568, 564049465049088)),
    DBB(u64x2::new(4539061024849920, 1128103225065472)),
    DBB(u64x2::new(9078117754732544, 2256206466908160)),
    DBB(u64x2::new(18155135997837312, 4512412933881856)),
    DBB(u64x2::new(36028797018963968, 9024825867763968)),
    DBB(u64x2::new(0, 18049651735527937)),
];
