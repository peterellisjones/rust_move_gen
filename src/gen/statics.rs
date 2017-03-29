use bb::*;

#[cfg(target_feature = "sse3")]
use dbb::DBB;

use square::Square;

#[cfg(target_feature = "sse3")]
extern crate simd;

#[cfg(target_feature = "sse3")]
use self::simd::x86::sse2::*;

pub const BISHOP_DIRECTIONS: [(u32, BB); 4] = [(9, BB(FILE_A.0 | ROW_1.0)), // up + right
                                               (7, BB(FILE_H.0 | ROW_1.0)), // up + left
                                               (64 - 9, BB(FILE_H.0 | ROW_8.0)), // down + left
                                               (64 - 7, BB(FILE_A.0 | ROW_8.0))]; // down + right

pub const ROOK_DIRECTIONS: [(u32, BB); 4] = [(1, FILE_A), // right
                                             (8, ROW_1), // up
                                             (64 - 1, FILE_H), // left
                                             (64 - 8, ROW_8)]; // down


pub const KNIGHT_DIRECTIONS: [(u32, BB); 8] =
    [(8 + 8 + 1, BB(FILE_A.0 | ROW_1.0 | ROW_2.0)), // up + up + right
     (8 + 1 + 1, BB(FILE_A.0 | FILE_B.0 | ROW_1.0)), // up + right + right
     ((-8 + 1 + 1) as u32, BB(FILE_A.0 | FILE_B.0 | ROW_8.0)), // down + right + right
     ((-8 - 8 + 1) as u32, BB(FILE_A.0 | ROW_8.0 | ROW_7.0)), // down + down + right
     ((-8 - 8 - 1) as u32, BB(FILE_H.0 | ROW_8.0 | ROW_7.0)), // down + down + left
     ((-8 - 1 - 1) as u32, BB(FILE_H.0 | FILE_G.0 | ROW_8.0)), // down + left + left
     (8 - 1 - 1, BB(FILE_H.0 | FILE_G.0 | ROW_1.0)), // up + left + left
     (8 + 8 - 1, BB(FILE_H.0 | ROW_1.0 | ROW_2.0))]; // up + up + left

pub static mut KING_MOVES: [BB; 64] = [BB(0); 64];
pub static mut KNIGHT_MOVES: [BB; 64] = [BB(0); 64];

#[cfg(not(target_feature = "sse3"))]
pub static mut DIAGONALS: [BB; 64] = [BB(0); 64];

#[cfg(not(target_feature = "sse3"))]
pub static mut ANTI_DIAGONALS: [BB; 64] = [BB(0); 64];

#[cfg(target_feature = "sse3")]
pub static mut BOTH_DIAGONALS: [DBB; 64] = [DBB(u64x2::new(0, 0)); 64];

#[derive(Copy, Clone)]
pub struct LineMask {
    pub upper: BB,
    pub lower: BB,
}

pub static mut BISHOP_LINE_MASKS: [[LineMask; 2]; 64] = [[LineMask {
    upper: EMPTY,
    lower: EMPTY,
}; 2]; 64];

// file; rank
pub static mut ROOK_LINE_MASKS: [[LineMask; 2]; 64] = [[LineMask {
    upper: EMPTY,
    lower: EMPTY,
}; 2]; 64];

// pub static mut RANK_ATTACKS: [[u8; 8]; 64] = [[0u8; 8]; 64];

pub fn init_all() {
    init_diagonals();
    init_rank_attacks();
    init_rook_line_masks();
    init_bishop_line_masks();
    init_king_moves();
    init_knight_moves();
}

fn init_rank_attacks() {
        // for i in 0..64 {
        //     for j in 0..8 {
        //         // unsafe {
        //         //     // RANK_ATTACKS[j][i] = 0;
        //         // }
        //     }
        // }
}

fn init_diagonals() {
    for i in 0..64 {
        let from = Square(i);
        let mut diag = EMPTY;
        let mut anti_diag = EMPTY;
        for (idx, &(shift, mask)) in BISHOP_DIRECTIONS.iter().enumerate() {
            let mut targets = BB::new(from).rot_left(shift);
            loop {
                if (targets & mask).any() {
                    break;
                }
                if idx % 2 == 0 {
                    diag |= targets;
                } else {
                    anti_diag |= targets;
                }
                targets |= targets.rot_left(shift);
            }
        }

        #[cfg(target_feature = "sse3")]
        unsafe {
            BOTH_DIAGONALS[i] = DBB(u64x2::new(diag.0, anti_diag.0));
        }

        #[cfg(not(target_feature = "sse3"))]
        unsafe {
            DIAGONALS[i] = diag;
        }

        #[cfg(not(target_feature = "sse3"))]
        unsafe {
            ANTI_DIAGONALS[i] = anti_diag;
        }
    }
}

fn init_rook_line_masks() {
    for i in 0..64 {
        let from = Square(i);
        let mut ret = [BB(0); 4];
        for (idx, &(shift, mask)) in ROOK_DIRECTIONS.iter().enumerate() {
            let mut targets = BB::new(from).rot_left(shift);
            loop {
                if (targets & mask).any() {
                    break;
                }
                ret[idx] |= targets;
                targets |= targets.rot_left(shift);
            }
        }

        let file = LineMask {
            upper: ret[0],
            lower: ret[2],
        };
        let rank = LineMask {
            upper: ret[1],
            lower: ret[3],
        };
        unsafe {
            ROOK_LINE_MASKS[i] = [file, rank];
        }
    }
}

fn init_bishop_line_masks() {
    for i in 0..64 {
        let from = Square(i);
        let mut ret = [BB(0); 4];
        for (idx, &(shift, mask)) in BISHOP_DIRECTIONS.iter().enumerate() {
            let mut targets = BB::new(from).rot_left(shift);
            loop {
                if (targets & mask).any() {
                    break;
                }
                ret[idx] |= targets;
                targets |= targets.rot_left(shift);
            }
        }

        let anti_diag = LineMask {
            upper: ret[1],
            lower: ret[3],
        };

        let diag = LineMask {
            upper: ret[0],
            lower: ret[2],
        };

        unsafe {
            BISHOP_LINE_MASKS[i] = [anti_diag, diag];
        }
    }
}

fn init_king_moves() {
    for i in 0..64 {
        let from = Square(i);
        let mut ret = BB(0);
        for &(shift, mask) in ROOK_DIRECTIONS.iter() {
            ret |= BB::new(from).rot_left(shift) & !mask;
        }
        for &(shift, mask) in BISHOP_DIRECTIONS.iter() {
            ret |= BB::new(from).rot_left(shift) & !mask;
        }

        unsafe {
            KING_MOVES[i] = ret;
        }
    }
}

fn init_knight_moves() {
    for i in 0..64 {
        let from = Square(i);
        let mut ret = BB(0);
        for &(shift, mask) in KNIGHT_DIRECTIONS.iter() {
            ret |= BB::new(from).rot_left(shift) & !mask;
        }

        unsafe {
            KNIGHT_MOVES[i] = ret;
        }
    }
}
