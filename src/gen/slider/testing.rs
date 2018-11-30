#![cfg(test)]

use super::ray_naive::{bishop_attacks_from_sq, rook_attacks_from_sq};
use bb::*;
use square;
use square::*;
use test;

const TEST_CASE_BOARD_DENSITY: f64 = 0.3;

pub fn test_bishop_attacks_from_bb<F: Fn(BB, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_bb(bishop_attacks_from_sq, TEST_CASE_BOARD_DENSITY);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn test_bishop_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_sq(bishop_attacks_from_sq, TEST_CASE_BOARD_DENSITY);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn test_rook_attacks_from_bb<F: Fn(BB, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_bb(rook_attacks_from_sq, TEST_CASE_BOARD_DENSITY);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn test_rook_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let cases = generate_test_cases_from_sq(rook_attacks_from_sq, TEST_CASE_BOARD_DENSITY);

    for (from, occupied, expected) in cases {
        let actual = gen(from, occupied);
        assert_eq!(actual, expected);
    }
}

pub fn bench_attacks_from_sq<F: Fn(Square, BB) -> BB>(b: &mut test::Bencher, gen: F) {
    let cases = random_occupancies_from_sq(640, TEST_CASE_BOARD_DENSITY);

    b.iter(|| -> BB {
        let mut ret = EMPTY;
        for &(sq, occupied) in cases.iter() {
            ret ^= gen(sq, occupied);
        }
        ret
    });
}

pub fn bench_attacks_from_bb<F: Fn(BB, BB) -> BB>(b: &mut test::Bencher, gen: F) {
    let cases = random_occupancies_from_bb(640, TEST_CASE_BOARD_DENSITY);
    b.iter(|| -> BB {
        let mut ret = EMPTY;

        for &(from, occupied) in cases.iter() {
            ret = ret ^ gen(from, occupied);
        }

        ret
    });
}

fn random_occupancies_from_bb(size: isize, density: f64) -> Vec<(BB, BB)> {
    let mut ret = Vec::new();
    for _ in 0..size {
        let sq1 = Square::random();
        let sq2 = Square::random();
        let from = BB::new(sq1) | BB::new(sq2);
        let occupied = BB::random(density) | from;
        ret.push((from, occupied));
    }
    ret
}

fn random_occupancies_from_sq(size: usize, density: f64) -> Vec<(Square, BB)> {
    let mut ret = Vec::new();
    for i in 0..size {
        let from = Square((i % 64) as square::Internal);
        let occupied = BB::random(density) | BB::new(from);
        ret.push((from, occupied));
    }
    ret
}

fn generate_test_cases_from_sq<F: Fn(Square, BB) -> BB>(
    gen: F,
    density: f64,
) -> Vec<(Square, BB, BB)> {
    let mut cases = Vec::new();
    for i in 0..64 {
        let from = Square::new(i);
        let occupied = BB::random(density) | BB::new(from);
        let expected = gen(from, occupied);
        cases.push((from, occupied, expected));
    }
    cases
}

fn generate_test_cases_from_bb<F: Fn(Square, BB) -> BB>(gen: F, density: f64) -> Vec<(BB, BB, BB)> {
    let mut cases = Vec::new();
    for i in 0..64 {
        let from_a = Square::new(i);
        let from_b = Square::random();
        let from = BB::new(from_a) | BB::new(from_b);
        let occupied = BB::random(density) | from;
        let expected = gen(from_a, occupied) | gen(from_b, occupied);
        cases.push((from, occupied, expected));
    }
    cases
}
