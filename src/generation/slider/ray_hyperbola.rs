// Hyperbola quientescence sliding piece attacks
// efficient for single sliders

use crate::bb::*;

#[cfg(target_feature = "sse3")]
use crate::dbb::*;

use crate::square::Square;

#[cfg(target_feature = "sse3")]
use std::simd::u64x2;

#[allow(dead_code)]
pub fn rook_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= file_attacks_from_sq(sq, occupied) | rank_attacks_from_sq(sq, occupied);
    }
    attacks
}

#[allow(dead_code)]
pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    file_attacks_from_sq(from, occupied) | rank_attacks_from_sq(from, occupied)
}

pub fn file_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let source = BB::new(from);

    let filemask = (FILE_A << from.col() as usize) & !source;
    let forward = filemask & occupied;
    let backward = forward.bswap();
    ((forward - source) ^ (backward - source.bswap()).bswap()) & filemask
}

pub fn rank_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let rowx8 = from.rowx8() as usize;
    let col = from.col() as usize;

    let occupancy = (occupied >> (rowx8 + 1)).to_usize() & 63;
    let attacks = unsafe { *RANK_ATTACKS.get_unchecked(occupancy).get_unchecked(col) };
    BB((attacks as u64) << rowx8)
}

#[allow(dead_code)]
pub fn bishop_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= bishop_attacks_from_sq(sq, occupied);
    }
    attacks
}

#[cfg(not(target_feature = "sse3"))]
pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let source = BB::new(from);

    let diag_mask = from.diagonals();
    let forward = diag_mask & occupied;
    let backward = forward.bswap();
    let diag_attacks = ((forward - source) ^ (backward - source.bswap()).bswap()) & diag_mask;

    let anti_diag_mask = from.anti_diagonals();
    let forward = anti_diag_mask & occupied;
    let backward = forward.bswap();
    let anti_diag_attacks =
        ((forward - source) ^ (backward - source.bswap()).bswap()) & anti_diag_mask;

    diag_attacks | anti_diag_attacks
}

#[cfg(target_feature = "sse3")]
pub fn bishop_attacks_from_sq(from: Square, occupied_bb: BB) -> BB {
    let source = DBB::splat(BB(1) << from.to_usize());
    let source_rev = source.bswap();
    let occupied = DBB::splat(occupied_bb);

    unsafe {
        let masks = *BOTH_DIAGONALS_DBB.get_unchecked(from.to_usize());
        let forward = masks & occupied;

        let backward = forward.bswap();

        let forward_targets = forward - source;
        let backward_targets = (backward - source_rev).bswap();

        let ret = (forward_targets ^ backward_targets) & masks;

        let (high, low) = ret.extract();
        high | low
    }
}

#[cfg(test)]
mod test {
    use super::super::testing::*;
    use super::*;
    use ::test;

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
    fn bench_multiple_rook_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, rook_attacks);
    }

    #[bench]
    fn bench_multiple_bishop_attacks(b: &mut test::Bencher) {
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

// 8 * 64 = 512 bytes
#[cfg(target_feature = "sse3")]
pub const BOTH_DIAGONALS_DBB: [DBB; 64] = [
    DBB(u64x2::from_array([0, 9241421688590303744])),
    DBB(u64x2::from_array([256, 36099303471055872])),
    DBB(u64x2::from_array([66048, 141012904183808])),
    DBB(u64x2::from_array([16909312, 550831656960])),
    DBB(u64x2::from_array([4328785920, 2151686144])),
    DBB(u64x2::from_array([1108169199616, 8404992])),
    DBB(u64x2::from_array([283691315109888, 32768])),
    DBB(u64x2::from_array([72624976668147712, 0])),
    DBB(u64x2::from_array([2, 4620710844295151616])),
    DBB(u64x2::from_array([65540, 9241421688590303233])),
    DBB(u64x2::from_array([16908296, 36099303471054850])),
    DBB(u64x2::from_array([4328783888, 141012904181764])),
    DBB(u64x2::from_array([1108169195552, 550831652872])),
    DBB(u64x2::from_array([283691315101760, 2151677968])),
    DBB(u64x2::from_array([72624976668131456, 8388640])),
    DBB(u64x2::from_array([145249953336262656, 64])),
    DBB(u64x2::from_array([516, 2310355422147510272])),
    DBB(u64x2::from_array([16778248, 4620710844295020800])),
    DBB(u64x2::from_array([4328523792, 9241421688590041601])),
    DBB(u64x2::from_array([1108168675360, 36099303470531586])),
    DBB(u64x2::from_array([283691314061376, 141012903135236])),
    DBB(u64x2::from_array([72624976666050688, 550829559816])),
    DBB(u64x2::from_array([145249953332101120, 2147491856])),
    DBB(u64x2::from_array([290499906664136704, 16416])),
    DBB(u64x2::from_array([132104, 1155177711056977920])),
    DBB(u64x2::from_array([4295231504, 2310355422114021376])),
    DBB(u64x2::from_array([1108102090784, 4620710844228043008])),
    DBB(u64x2::from_array([283691180892224, 9241421688456086017])),
    DBB(u64x2::from_array([72624976399712384, 36099303202620418])),
    DBB(u64x2::from_array([145249952799424512, 141012367312900])),
    DBB(u64x2::from_array([290499905598783488, 549757915144])),
    DBB(u64x2::from_array([580999811180789760, 4202512])),
    DBB(u64x2::from_array([33818640, 577588851233521664])),
    DBB(u64x2::from_array([1099579265056, 1155177702483820544])),
    DBB(u64x2::from_array([283674135240768, 2310355404967706624])),
    DBB(u64x2::from_array([72624942308409472, 4620710809935413504])),
    DBB(u64x2::from_array([145249884616818688, 9241421619870827009])),
    DBB(u64x2::from_array([290499769233571840, 36099166032102402])),
    DBB(u64x2::from_array([580999538450366464, 140738026276868])),
    DBB(u64x2::from_array([1161999072605765632, 1075843080])),
    DBB(u64x2::from_array([8657571872, 288793326105133056])),
    DBB(u64x2::from_array([281492291854400, 577586656505233408])),
    DBB(u64x2::from_array([72620578621636736, 1155173313027244032])),
    DBB(u64x2::from_array([145241157243273216, 2310346626054553600])),
    DBB(u64x2::from_array([290482314486480896, 4620693252109107456])),
    DBB(u64x2::from_array([580964628956184576, 9241386504218214913])),
    DBB(u64x2::from_array([1161929253617401856, 36028934726878210])),
    DBB(u64x2::from_array([2323857407723175936, 275415828484])),
    DBB(u64x2::from_array([2216338399296, 144115188075855872])),
    DBB(u64x2::from_array([72062026714726528, 288231475663339520])),
    DBB(u64x2::from_array([144124053429452800, 576462955621646336])),
    DBB(u64x2::from_array([288248106858840064, 1152925911260069888])),
    DBB(u64x2::from_array([576496213700902912, 2305851822520205312])),
    DBB(u64x2::from_array([
        1152992423106838528,
        4611703645040410880,
    ])),
    DBB(u64x2::from_array([
        2305983746702049280,
        9223407290080821761,
    ])),
    DBB(u64x2::from_array([4611686018427387904, 70506452091906])),
    DBB(u64x2::from_array([567382630219904, 0])),
    DBB(u64x2::from_array([1134765260439552, 281474976710656])),
    DBB(u64x2::from_array([2269530520813568, 564049465049088])),
    DBB(u64x2::from_array([4539061024849920, 1128103225065472])),
    DBB(u64x2::from_array([9078117754732544, 2256206466908160])),
    DBB(u64x2::from_array([18155135997837312, 4512412933881856])),
    DBB(u64x2::from_array([36028797018963968, 9024825867763968])),
    DBB(u64x2::from_array([0, 18049651735527937])),
];
