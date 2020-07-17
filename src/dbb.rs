// Represents a double bitboard
use std::ops::*;

#[cfg(target_feature = "sse3")]
extern crate packed_simd;
#[cfg(target_feature = "sse3")]
use self::packed_simd::*;

use bb::BB;
use std::mem::transmute;

/// Double bitboard used with SSE3 intrinsics
#[derive(Copy, Clone)]
pub struct DBB(pub u64x2);

impl Sub for DBB {
    type Output = DBB;

    #[inline]
    fn sub(self, other: DBB) -> DBB {
        DBB(self.0 - other.0)
    }
}

impl BitAnd for DBB {
    type Output = DBB;

    #[inline]
    fn bitand(self, other: DBB) -> DBB {
        DBB(self.0 & other.0)
    }
}

impl BitOr for DBB {
    type Output = DBB;

    #[inline]
    fn bitor(self, other: DBB) -> DBB {
        DBB(self.0 | other.0)
    }
}

impl BitXor for DBB {
    type Output = DBB;

    #[inline]
    fn bitxor(self, other: DBB) -> DBB {
        DBB(self.0 ^ other.0)
    }
}

impl Shl<u32> for DBB {
    type Output = DBB;

    #[inline]
    fn shl(self, amount: u32) -> DBB {
        DBB(self.0 << amount)
    }
}

const NOT_FILE_A: DBB = DBB(u64x2::new(!0x0101010101010101u64, !0x0101010101010101u64));
const NOT_FILE_H: DBB = DBB(u64x2::new(
    !(0x0101010101010101u64 << 7),
    !(0x0101010101010101u64 << 7),
));

impl DBB {
    #[inline]
    pub fn new(a: BB, b: BB) -> DBB {
        DBB(u64x2::new(a.to_u64(), b.to_u64()))
    }

    #[inline]
    pub fn splat(source: BB) -> DBB {
        DBB(u64x2::splat(source.to_u64()))
    }

    #[inline]
    pub fn extract(&self) -> (BB, BB) {
        (BB(self.0.extract(0)), BB(self.0.extract(1)))
    }

    #[inline]
    pub fn bswap(&self) -> DBB {
        let bytes: u8x16 = unsafe { transmute(self.0) };
        let shuffled: u8x16 = shuffle!(
            bytes,
            [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]
        );
        let ret: u64x2 = unsafe { transmute(shuffled) };
        DBB(ret)
    }

    #[inline]
    pub fn occluded_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen << 1));
        prop = prop & (prop << 1);
        gen = gen | (prop & (gen << 2));
        prop = prop & (prop << 2);
        gen = gen | (prop & (gen << 4));

        DBB(gen)
    }

    #[inline]
    pub fn east_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_east_fill(empty);

        DBB(gen.0 << 1) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_north_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen << 9));
        prop = prop & (prop << 9);
        gen = gen | (prop & (gen << 18));
        prop = prop & (prop << 18);
        gen = gen | (prop & (gen << 36));

        DBB(gen)
    }

    #[inline]
    pub fn north_east_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_east_fill(empty);

        DBB(gen.0 << 9) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_north_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen << 8));
        prop = prop & (prop << 8);
        gen = gen | (prop & (gen << 16));
        prop = prop & (prop << 16);
        gen = gen | (prop & (gen << 32));

        DBB(gen)
    }

    #[inline]
    pub fn north_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_fill(empty);

        DBB(gen.0 << 8)
    }

    #[inline]
    pub fn occluded_south_east_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_A.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen >> 7));
        prop = prop & (prop >> 7);
        gen = gen | (prop & (gen >> 14));
        prop = prop & (prop >> 14);
        gen = gen | (prop & (gen >> 28));

        DBB(gen)
    }

    #[inline]
    pub fn south_east_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_east_fill(empty);

        DBB(gen.0 >> 7) & NOT_FILE_A
    }

    #[inline]
    pub fn occluded_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen >> 1));
        prop = prop & (prop >> 1);
        gen = gen | (prop & (gen >> 2));
        prop = prop & (prop >> 2);
        gen = gen | (prop & (gen >> 4));

        DBB(gen)
    }

    #[inline]
    pub fn west_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_west_fill(empty);

        DBB(gen.0 >> 1) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_south_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen >> 9));
        prop = prop & (prop >> 9);
        gen = gen | (prop & (gen >> 18));
        prop = prop & (prop >> 18);
        gen = gen | (prop & (gen >> 36));

        DBB(gen)
    }

    #[inline]
    pub fn south_west_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_west_fill(empty);

        DBB(gen.0 >> 9) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_north_west_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0 & NOT_FILE_H.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen << 7));
        prop = prop & (prop << 7);
        gen = gen | (prop & (gen << 14));
        prop = prop & (prop << 14);
        gen = gen | (prop & (gen << 28));

        DBB(gen)
    }

    #[inline]
    pub fn north_west_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_west_fill(empty);

        DBB(gen.0 << 7) & NOT_FILE_H
    }

    #[inline]
    pub fn occluded_south_fill(&self, empty: DBB) -> DBB {
        let mut prop = empty.0;
        let mut gen = self.0;

        gen = gen | (prop & (gen >> 8));
        prop = prop & (prop >> 8);
        gen = gen | (prop & (gen >> 16));
        prop = prop & (prop >> 16);
        gen = gen | (prop & (gen >> 32));

        DBB(gen)
    }

    #[inline]
    pub fn south_attacks(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_fill(empty);

        DBB(gen.0 >> 8)
    }

    #[inline]
    pub fn occluded_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_east_fill(empty);

        DBB(gen.0 | ((gen.0 << 1) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_north_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_east_fill(empty);

        DBB(gen.0 | ((gen.0 << 9) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_north_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_fill(empty);

        DBB(gen.0 | (gen.0 << 8))
    }

    #[inline]
    pub fn occluded_south_east_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_east_fill(empty);

        DBB(gen.0 | ((gen.0 >> 7) & NOT_FILE_A.0))
    }

    #[inline]
    pub fn occluded_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_west_fill(empty);

        DBB(gen.0 | ((gen.0 >> 1) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_south_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_west_fill(empty);

        DBB(gen.0 | ((gen.0 >> 9) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_north_west_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_north_west_fill(empty);

        DBB(gen.0 | ((gen.0 << 7) & NOT_FILE_H.0))
    }

    #[inline]
    pub fn occluded_south_fill_with_occluders(&self, empty: DBB) -> DBB {
        let gen = self.occluded_south_fill(empty);

        DBB(gen.0 | (gen.0 >> 8))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bswap() {
        //bswap should reverse bytes in each u64
        let input = DBB(u64x2::new(!0x0102030405060708u64, !0x0203040506070809u64));
        let (expected_left, expected_right) =
            (BB(!0x0807060504030201u64), BB(!0x0908070605040302u64));
        let (output_left, output_right) = input.bswap().extract();
        assert_eq!(output_right, expected_right);
        assert_eq!(output_left, expected_left);
    }
}
