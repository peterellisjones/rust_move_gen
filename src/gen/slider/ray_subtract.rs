// subtraction-based sliding piece attacks
// NOT IMPLEMENTED
// https://chessprogramming.wikispaces.com/SBAMG
use bb::*;
use square::Square;
use gen::statics::*;

#[allow(dead_code)]
fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let masks = unsafe { *ROOK_LINE_MASKS.get_unchecked(from.to_usize()) };
    let file_attacks = line_attacks(occupied, masks[0]);
    let rank_attacks = line_attacks(occupied, masks[1]);
    file_attacks | rank_attacks
}

#[allow(dead_code)]
fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let masks = unsafe { *BISHOP_LINE_MASKS.get_unchecked(from.to_usize()) };
    let diag_attacks = line_attacks(occupied, masks[0]);
    let antidiag_attacks = line_attacks(occupied, masks[1]);
    diag_attacks | antidiag_attacks
}

fn line_attacks(occupied: BB, mask: LineMask) -> BB {
    let lower = mask.lower & occupied;
    let upper = mask.upper & occupied;

    let msb = (-BB(1)) << ((if lower == EMPTY { BB(1) } else { lower }).msb() as usize);
    let lsb = upper & -upper;
    let diff = BB(2) * lsb + msb;
    let attacks = diff & (mask.lower | mask.upper);
    attacks
}

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
}