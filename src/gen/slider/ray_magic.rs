// magic-lookup sliding piece attacks
// NOT IMPLEMENTED
// https://chessprogramming.wikispaces.com/BMI2#PEXTBitboards

use bb::{BB, EMPTY};
use square::Square;

#[derive(Copy, Clone)]
struct Magic {
    magic_number: BB,
    mask: BB,
    offset: u32,
    rightshift: u8,
}

static mut MAGICS: [[Magic; 64]; 2] = [[Magic {
    magic_number: EMPTY,
    mask: EMPTY,
    offset: 0,
    rightshift: 0,
}; 64]; 2];

fn initialize_magics() {}

#[allow(dead_code)]
fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {

    EMPTY
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::testing::*;
    use test;

    // #[test]
    // fn t_rook_attacks() {
    //     test_rook_attacks_from_sq(rook_attacks_from_sq);
    // }

    // #[test]
    // fn t_bishop_attacks() {
    //     test_bishop_attacks_from_sq(bishop_attacks_from_sq);
    // }

    // #[bench]
    // fn bench_rook_attacks_from_sq(b: &mut test::Bencher) {
    //     bench_attacks_from_sq(b, rook_attacks_from_sq);
    // }

    // #[bench]
    // fn bench_bishop_attacks_from_sq(b: &mut test::Bencher) {
    //     bench_attacks_from_sq(b, bishop_attacks_from_sq);
    // }
}