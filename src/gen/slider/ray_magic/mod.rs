// magic-lookup sliding piece attacks
// https://chessprogramming.wikispaces.com/BMI2#PEXTBitboards

use bb::*;
use square::Square;

mod consts;

#[derive(Copy, Clone)]
pub struct Magic {
    magic_number: u64,
    mask: BB,
    offset: u32,
}

pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let magic = unsafe { *consts::BISHOP_MAGICS.get_unchecked(from.to_usize()) };
    let mult = (occupied & magic.mask)
        .to_u64()
        .wrapping_mul(magic.magic_number);
    let index = (mult >> 55) as usize;
    let offset = index + (magic.offset as usize);

    unsafe { *consts::SHARED_ATTACKS.get_unchecked(offset) }
}

pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let magic = unsafe { *consts::ROOK_MAGICS.get_unchecked(from.to_usize()) };
    let mult = (occupied & magic.mask)
        .to_u64()
        .wrapping_mul(magic.magic_number);
    let index = (mult >> 52) as usize;
    let offset = index + (magic.offset as usize);

    unsafe { *consts::SHARED_ATTACKS.get_unchecked(offset) }
}

#[allow(dead_code)]
pub fn rook_attacks(from: BB, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for (sq, _) in from.iter() {
        attacks |= rook_attacks_from_sq(sq, occupied);
    }
    attacks
}

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
    fn bench_multiple_rook_attacks(b: &mut test::Bencher) {
        bench_attacks_from_bb(b, rook_attacks);
    }

    #[bench]
    fn bench_multiple_bishop_attacks(b: &mut test::Bencher) {
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
