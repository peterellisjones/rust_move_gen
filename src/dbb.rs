// Represents a double bitboard
use std::{
    ops::*,
    simd::{simd_swizzle, u64x2, u8x16},
};

use crate::bb::BB;
use std::mem::transmute;

/// Double bitboard used with SSE3 intrinsics
#[derive(Copy, Clone)]
pub struct DBB(pub u64x2);

impl Sub for DBB {
    type Output = DBB;

    fn sub(self, other: DBB) -> DBB {
        DBB(self.0 - other.0)
    }
}

impl BitAnd for DBB {
    type Output = DBB;

    fn bitand(self, other: DBB) -> DBB {
        DBB(self.0 & other.0)
    }
}

impl BitOr for DBB {
    type Output = DBB;

    fn bitor(self, other: DBB) -> DBB {
        DBB(self.0 | other.0)
    }
}

impl BitXor for DBB {
    type Output = DBB;

    fn bitxor(self, other: DBB) -> DBB {
        DBB(self.0 ^ other.0)
    }
}

impl Shl<u64> for DBB {
    type Output = DBB;

    fn shl(self, amount: u64) -> DBB {
        DBB(self.0 << amount)
    }
}

const NOT_FILE_A: DBB = DBB(u64x2::from_array([
    !0x0101010101010101u64,
    !0x0101010101010101u64,
]));
const NOT_FILE_H: DBB = DBB(u64x2::from_array([
    !(0x0101010101010101u64 << 7),
    !(0x0101010101010101u64 << 7),
]));

impl DBB {
    pub fn new(a: BB, b: BB) -> DBB {
        DBB(u64x2::from_array([a.to_u64(), b.to_u64()]))
    }

    pub fn splat(source: BB) -> DBB {
        DBB(u64x2::splat(source.to_u64()))
    }

    pub fn extract(&self) -> (BB, BB) {
        let arr = self.0.as_array();
        (BB(arr[0]), BB(arr[1]))
    }

    pub fn bswap(&self) -> DBB {
        let bytes: u8x16 = unsafe { transmute(self.0) };
        let shuffled: u8x16 = simd_swizzle!(
            bytes,
            [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        );
        let ret: u64x2 = unsafe { transmute(shuffled) };
        DBB(ret)
    }

    pub fn occluded_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator << 1));
        prop = prop & (prop << 1);
        generator = generator | (prop & (generator << 2));
        prop = prop & (prop << 2);
        generator = generator | (prop & (generator << 4));

        DBB(generator)
    }

    pub fn east_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_east_fill(empty);

        DBB(generator.0 << 1) & NOT_FILE_A
    }

    pub fn occluded_north_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator << 9));
        prop = prop & (prop << 9);
        generator = generator | (prop & (generator << 18));
        prop = prop & (prop << 18);
        generator = generator | (prop & (generator << 36));

        DBB(generator)
    }

    pub fn north_east_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_east_fill(empty);

        DBB(generator.0 << 9) & NOT_FILE_A
    }

    pub fn occluded_north_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator << 8));
        prop = prop & (prop << 8);
        generator = generator | (prop & (generator << 16));
        prop = prop & (prop << 16);
        generator = generator | (prop & (generator << 32));

        DBB(generator)
    }

    pub fn north_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_fill(empty);

        DBB(generator.0 << 8)
    }

    pub fn occluded_south_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator >> 7));
        prop = prop & (prop >> 7);
        generator = generator | (prop & (generator >> 14));
        prop = prop & (prop >> 14);
        generator = generator | (prop & (generator >> 28));

        DBB(generator)
    }

    pub fn south_east_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_east_fill(empty);

        DBB(generator.0 >> 7) & NOT_FILE_A
    }

    pub fn occluded_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator >> 1));
        prop = prop & (prop >> 1);
        generator = generator | (prop & (generator >> 2));
        prop = prop & (prop >> 2);
        generator = generator | (prop & (generator >> 4));

        DBB(generator)
    }

    pub fn west_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_west_fill(empty);

        DBB(generator.0 >> 1) & NOT_FILE_H
    }

    pub fn occluded_south_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator >> 9));
        prop = prop & (prop >> 9);
        generator = generator | (prop & (generator >> 18));
        prop = prop & (prop >> 18);
        generator = generator | (prop & (generator >> 36));

        DBB(generator)
    }

    pub fn south_west_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_west_fill(empty);

        DBB(generator.0 >> 9) & NOT_FILE_H
    }

    pub fn occluded_north_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator << 7));
        prop = prop & (prop << 7);
        generator = generator | (prop & (generator << 14));
        prop = prop & (prop << 14);
        generator = generator | (prop & (generator << 28));

        DBB(generator)
    }

    pub fn north_west_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_west_fill(empty);

        DBB(generator.0 << 7) & NOT_FILE_H
    }

    pub fn occluded_south_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0;
        let mut generator = self.0;

        generator = generator | (prop & (generator >> 8));
        prop = prop & (prop >> 8);
        generator = generator | (prop & (generator >> 16));
        prop = prop & (prop >> 16);
        generator = generator | (prop & (generator >> 32));

        DBB(generator)
    }

    pub fn south_attacks(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_fill(empty);

        DBB(generator.0 >> 8)
    }

    pub fn occluded_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_east_fill(empty);

        DBB(generator.0 | ((generator.0 << 1) & NOT_FILE_A.0))
    }

    pub fn occluded_north_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_east_fill(empty);

        DBB(generator.0 | ((generator.0 << 9) & NOT_FILE_A.0))
    }

    pub fn occluded_north_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_fill(empty);

        DBB(generator.0 | (generator.0 << 8))
    }

    pub fn occluded_south_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_east_fill(empty);

        DBB(generator.0 | ((generator.0 >> 7) & NOT_FILE_A.0))
    }

    pub fn occluded_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_west_fill(empty);

        DBB(generator.0 | ((generator.0 >> 1) & NOT_FILE_H.0))
    }

    pub fn occluded_south_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_west_fill(empty);

        DBB(generator.0 | ((generator.0 >> 9) & NOT_FILE_H.0))
    }

    pub fn occluded_north_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_north_west_fill(empty);

        DBB(generator.0 | ((generator.0 << 7) & NOT_FILE_H.0))
    }

    pub fn occluded_south_fill_with_occluders(&self, empty: DBB) -> DBB {
        let generator = self.occluded_south_fill(empty);

        DBB(generator.0 | (generator.0 >> 8))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bswap() {
        //bswap should reverse bytes in each u64
        let input = DBB(u64x2::from_array([
            !0x0102030405060708u64,
            !0x0203040506070809u64,
        ]));
        let (expected_left, expected_right) =
            (BB(!0x0807060504030201u64), BB(!0x0908070605040302u64));
        let (output_left, output_right) = input.bswap().extract();
        assert_eq!(output_right, expected_right);
        assert_eq!(output_left, expected_left);
    }
}
