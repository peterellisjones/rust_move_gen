// Hyperbola quientescence sliding piece attacks
// efficient for single sliders

use bb::*;
use square::Square;
#[cfg(target_feature = "sse3")]
use dbb::*;

#[cfg(target_feature = "sse3")]
extern crate simd;

use gen::statics::*;

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
    unsafe {
        let attacks = *RANK_ATTACKS.get_unchecked(occupancy).get_unchecked(col);
        return BB((attacks as u64) << rowx8);
    }
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
    let anti_diag_attacks = ((forward - source) ^ (backward - source.bswap()).bswap()) &
                            anti_diag_mask;

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

/// Indexed by source square and 6-bit occupancy. 8 * 64 = 512 bytes
const RANK_ATTACKS: [[u8; 8]; 64] = [[254, 253, 251, 247, 239, 223, 191, 127],
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
                                     [2, 5, 10, 20, 40, 80, 160, 64]];


#[cfg(test)]
mod test {
    use super::*;
    use super::super::testing::*;
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