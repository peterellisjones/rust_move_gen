use bb::*;
use square::Square;

mod consts;

#[cfg(test)]
mod gen;

#[cfg(test)]
pub use self::gen::generate_offsets;

use self::consts::*;
use std::arch::x86_64::{_pdep_u64, _pext_u64};

#[inline]
pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    return unsafe {
        let offset = BISHOP_OFFSETS.get_unchecked(from.to_usize());

        let idx =
            (offset.base_offset as usize) + (_pext_u64(occupied.0, offset.inner_mask.0) as usize);

        let attack_indexes = *SHARED_ATTACK_INDICES.get_unchecked(idx);

        BB(_pdep_u64(attack_indexes as u64, offset.outer_mask.0))
    };
}

#[inline]
pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    return unsafe {
        let offset = ROOK_OFFSETS.get_unchecked(from.to_usize());

        let idx =
            (offset.base_offset as usize) + (_pext_u64(occupied.0, offset.inner_mask.0) as usize);

        let attack_indexes = *SHARED_ATTACK_INDICES.get_unchecked(idx);

        BB(_pdep_u64(attack_indexes as u64, offset.outer_mask.0))
    };
}

#[inline]
#[allow(dead_code)]
pub fn rook_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= rook_attacks_from_sq(sq, occupied);
    }
    attacks
}

#[inline]
#[allow(dead_code)]
pub fn bishop_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= rook_attacks_from_sq(sq, occupied);
    }
    attacks
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
    fn bench_rook_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, rook_attacks);
    }

    #[bench]
    fn bench_bishop_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, bishop_attacks);
    }

    #[bench]
    fn bench_rook_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, bishop_attacks_from_sq);
    }
}
