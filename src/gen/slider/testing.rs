
#![cfg(test)]

use bb::*;
use square::*;
use unindent;
use test;
use gen::statics::{ROOK_DIRECTIONS, BISHOP_DIRECTIONS};

pub fn test_bishop_attacks_from_bb<F: Fn(BB, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_bb(naive_bishop_attacks_from_sq);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn test_bishop_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_sq(naive_bishop_attacks_from_sq);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn test_rook_attacks_from_bb<F: Fn(BB, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_bb(naive_rook_attacks_from_sq);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}
pub fn test_rook_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_sq(naive_rook_attacks_from_sq);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn bench_attacks_from_bb<F: Fn(BB, BB) -> BB>(b: &mut test::Bencher, gen: F) {
    let cases = random_occupancies_from_bb(100);
    b.iter(|| -> BB {
        let mut ret = EMPTY;

        for &(from, occupied) in cases.iter() {
            ret = ret ^ gen(from, occupied);
        }

        ret
    });
}

pub fn bench_attacks_from_sq<F: Fn(Square, BB) -> BB>(b: &mut test::Bencher, gen: F) {
    let cases = random_occupancies_from_sq(100);
    b.iter(|| -> BB {
        let mut ret = EMPTY;

        for &(from, occupied) in cases.iter() {
            ret ^= gen(from, occupied);
        }

        ret
    });
}

fn random_occupancies_from_bb(size: isize) -> Vec<(BB, BB)> {
    let mut ret = Vec::new();
    for _ in 0..size {
        let sq1 = Square::random();
        let sq2 = Square::random();
        let from = BB::new(sq1) | BB::new(sq2);
        let occupied = BB::random(0.3) | from;
        ret.push((from, occupied));
    }
    ret
}

fn random_occupancies_from_sq(size: usize) -> Vec<(Square, BB)> {
    let mut ret = Vec::new();
    for i in 0..size {
        let from = Square(i % 64);
        let occupied = BB::random(0.3) | BB::new(from);
        ret.push((from, occupied));
    }
    ret
}

fn naive_rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let mut attacks = EMPTY;
    for &(shift, mask) in ROOK_DIRECTIONS.iter() {
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


fn naive_bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
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

fn generate_test_cases_from_sq<F: Fn(Square, BB) -> BB>(gen: F) -> Vec<(Square, BB, BB)> {
    let mut cases = Vec::new();
    for i in 0..64 {
        let from = Square::new(i);
        let occupied = BB::random(0.3) | BB::new(from);
        let expected = gen(from, occupied);
        cases.push((from, occupied, expected));
    }
    cases
}

fn generate_test_cases_from_bb<F: Fn(Square, BB) -> BB>(gen: F) -> Vec<(BB, BB, BB)> {
    let mut cases = Vec::new();
    for i in 0..64 {
        let from_a = Square::new(i);
        let from_b = Square::random();
        let from = BB::new(from_a) | BB::new(from_b);
        let occupied = BB::random(0.3) | from;
        let expected = gen(from_a, occupied) | gen(from_b, occupied);
        cases.push((from, occupied, expected));
    }
    cases
}

#[bench]
fn bench_naive_rook_attacks_from_sq(b: &mut test::Bencher) {
    bench_attacks_from_sq(b, naive_rook_attacks_from_sq);
}

#[bench]
fn bench_naive_bishop_attacks_from_sq(b: &mut test::Bencher) {
    bench_attacks_from_sq(b, naive_bishop_attacks_from_sq);
}