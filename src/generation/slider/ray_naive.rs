use crate::bb::*;
use crate::square::Square;

#[allow(dead_code)]
pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    const ROOK_DIRECTIONS: [(u32, BB); 4] = [
        (1, FILE_A),      // right
        (8, ROW_1),       // up
        (64 - 1, FILE_H), // left
        (64 - 8, ROW_8),
    ]; // down

    let mut attacks = EMPTY;
    for &(shift, mask) in ROOK_DIRECTIONS.iter() {
        let mut targets = BB::new(from).rot_left(shift);
        loop {
            if (targets & (mask | occupied)).any() {
                break;
            }
            targets |= targets.rot_left(shift);
        }
        attacks |= targets & !mask;
    }
    attacks
}

#[allow(dead_code)]
pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    const BISHOP_DIRECTIONS: [(u32, BB); 4] = [
        (9, BB(FILE_A.0 | ROW_1.0)),      // up + right
        (7, BB(FILE_H.0 | ROW_1.0)),      // up + left
        (64 - 9, BB(FILE_H.0 | ROW_8.0)), // down + left
        (64 - 7, BB(FILE_A.0 | ROW_8.0)),
    ]; // down + right

    let mut attacks = EMPTY;
    for &(shift, mask) in BISHOP_DIRECTIONS.iter() {
        let mask_or_occupied = mask | occupied;
        let mut targets = BB::new(from).rot_left(shift);
        loop {
            if (targets & mask_or_occupied).any() {
                break;
            }
            targets |= targets.rot_left(shift);
        }
        attacks |= targets & !mask;
    }
    attacks
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
    use ::test;

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
