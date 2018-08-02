use bb::*;
use square::Square;

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

#[cfg(test)]
mod test {
    use super::super::testing::*;
    use super::*;
    use test;

    // #[test]
    // fn print_cases() {
    //     let (mut cases, bishop_hash, rook_hash) = generate_all_test_cases(bishop_attacks_from_sq, rook_attacks_from_sq, 0.3);
    //     println!("const TEST_CASES_BISHOP_HASH: BB = BB(0x{:X})", bishop_hash.to_u64());
    //     println!("const TEST_CASES_ROOK_HASH: BB = BB(0x{:X})", rook_hash.to_u64());
    //     println!("const TEST_CASES: [(Square, BB); 640] = [");

    //     let mut rng = thread_rng();
    //     rng.shuffle(&mut cases);

    //     for &(sq, bb) in cases.iter() {
    //         println!("({}, 0x{:X}),", sq.to_usize(), bb.to_u64());
    //     }
    //     println!("];");

    //     assert!(false);
    // }

    #[bench]
    fn bench_rook_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, bishop_attacks_from_sq);
    }
}
