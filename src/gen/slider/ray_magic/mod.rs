// magic-lookup sliding piece attacks
// NOT IMPLEMENTED
// https://chessprogramming.wikispaces.com/BMI2#PEXTBitboards

use bb::*;
use square::Square;
use gen::statics::{ROOK_LINE_MASKS, DIAGONALS, ANTI_DIAGONALS};
use super::ray_subtract;
use rand;

mod consts;

#[derive(Copy, Clone)]
pub struct Magic {
    magic_number: u64,
    mask: BB,
    offset: u32,
}


pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let magic = consts::BISHOP_MAGICS[from.to_usize()];
    let mult = (occupied & magic.mask).to_u64().wrapping_mul(magic.magic_number);
    let index = (mult >>55) as usize;
    let offset = index + (magic.offset as usize);

    consts::SHARED_ATTACKS[offset]
}

pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let magic = consts::ROOK_MAGICS[from.to_usize()];
    let mult = (occupied & magic.mask).to_u64().wrapping_mul(magic.magic_number);
    let index = (mult >>52) as usize;
    let offset = index + (magic.offset as usize);

    consts::SHARED_ATTACKS[offset]
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

    #[bench]
    fn bench_rook_attacks_from_sq_low_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_low_density(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq_low_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_low_density(b, bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks_from_sq_high_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_high_density(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq_high_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_high_density(b, bishop_attacks_from_sq);
    }
}

#[allow(dead_code)]
fn print_magics(rook_magics: [Magic; 64], bishop_magics: [Magic; 64], attacks: Vec<BB>) {
  println!("");
  println!("pub const BISHOP_MAGICS: [Magic; 64] = [");
    for magic in bishop_magics.iter() {
        println!("  Magic {{");
        println!("    magic_number: 0x{:x},", magic.magic_number);
        println!("    mask: BB(0x{:x}),", magic.mask.to_u64());
        println!("    offset: {},", magic.offset);
        println!("  }},");
    }
    println!("];");
    println!("");
    println!("pub const ROOK_MAGICS: [Magic; 64] = [");
    for magic in rook_magics.iter() {
        println!("  Magic {{");
        println!("    magic_number: 0x{:x},", magic.magic_number);
        println!("    mask: BB(0x{:x}),", magic.mask.to_u64());
        println!("    offset: {},", magic.offset);
        println!("  }},");
    }
    println!("];");
    println!("");
    println!("pub const SHARED_ATTACKS: [BB; {}] = [", attacks.len());
    for attack in attacks.iter() {
        println!("  BB(0x{:x}),", attack.to_u64());
    }
    println!("];");
}

// http://www.talkchess.com/forum/viewtopic.php?t=60065&start=14
#[allow(dead_code)]
fn generate_shared_magics() -> ([Magic; 64],[Magic; 64], Vec<BB>) {
  let mut magic_attacks:  [BB; 79462] = [EMPTY; 79462];
  let mut bishop_magics = [Magic {
      magic_number: 0,
      mask: EMPTY,
      offset: 0,
  }; 64];

  for i in 0..64 {
    let sq = Square(i);
    let bishop_mask = (ANTI_DIAGONALS[i] | DIAGONALS[i]) & !(FILE_A | FILE_H | ROW_1 | ROW_8);
    let occupancy_bits = bishop_mask.pop_count() as usize;
    let occupancy_attack_pairs: Vec<(BB, BB)> = generate_occupancy_attack_pairs(sq, bishop_mask, ray_subtract::bishop_attacks_from_sq);

    let (magic_number, offset) = BISHOP_MAGICS_RAW[i];

    for &(occupancy, attacks) in occupancy_attack_pairs.iter() {
      let index = (occupancy.to_u64().wrapping_mul(magic_number) >> 55) as usize;
      magic_attacks[index + offset as usize] = attacks;
    }

    bishop_magics[i] = Magic {
        magic_number: magic_number,
        mask: bishop_mask,
        offset: offset,
    };
  }

  let mut rook_magics = [Magic {
      magic_number: 0,
      mask: EMPTY,
      offset: 0,
  }; 64];

  for i in 0..64 {
    let sq = Square(i);
    let mut rook_mask = ROOK_LINE_MASKS[i][0].lower | ROOK_LINE_MASKS[i][0].upper |  ROOK_LINE_MASKS[i][1].lower | ROOK_LINE_MASKS[i][1].upper;
    for edge in [FILE_A, FILE_H, ROW_1, ROW_8].iter() {
        if (BB::new(sq) & *edge).none() {
            rook_mask &= !*edge;
        }
    }

    let occupancy_bits = rook_mask.pop_count() as usize;
    let occupancy_attack_pairs: Vec<(BB, BB)> = generate_occupancy_attack_pairs(sq, rook_mask, ray_subtract::rook_attacks_from_sq);

    let (magic_number, offset) =  ROOK_MAGICS_RAW[i];

    for &(occupancy, attacks) in occupancy_attack_pairs.iter() {
      let index = (occupancy.to_u64().wrapping_mul(magic_number) >> 52) as usize;
      magic_attacks[index + offset as usize] = attacks;
    }

    rook_magics[i] = Magic {
        magic_number: magic_number,
        mask: rook_mask,
        offset: offset,
    };
  }

  let magic_attacks_vec = magic_attacks.iter().map(|bb| *bb).collect();

  (rook_magics, bishop_magics, magic_attacks_vec)
}

#[allow(dead_code)]
fn generate_occupancy_attack_pairs<F: Fn(Square, BB) -> BB>(sq: Square, mask: BB, move_gen: F) -> Vec<(BB, BB)> {
    let mut occupancy_attack_pairs: Vec<(BB, BB)> = Vec::new();
    let num_bits = mask.pop_count() as usize;

    let occupancy_squares: Vec<BB> = mask.iter().map(|(_, bb)| bb).collect();

    // eg 0..1024
    for i in 0..(1 << num_bits) {
        let mut occupancy = EMPTY;
        // eg 0 .. 10
        for j in 0..num_bits {
            if ((1 << j) & i) != 0 {
                occupancy |= occupancy_squares[j];
            }
        }
        let attacks = move_gen(sq, occupancy);
        occupancy_attack_pairs.push((occupancy, attacks));
    }

    occupancy_attack_pairs
}

#[allow(dead_code)]
fn find_magic(occupancy_bits: usize, occupancy_attack_pairs: &Vec<(BB, BB)>) -> u64 {
    let mut used_by: Vec<BB> = occupancy_attack_pairs.iter().map(|_| EMPTY ).collect();
    assert!(occupancy_attack_pairs.len() > 0);

     loop {
        for i in 0..(1<< occupancy_bits) {
            used_by[i] = EMPTY;
        }
        let attempt = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
        let mut fail = false;

        for &(occupancy, attacks) in occupancy_attack_pairs.iter() {
            let index = (occupancy.to_u64().wrapping_mul(attempt) >> ( 64-occupancy_bits)) as usize;
            if used_by[index] != EMPTY && used_by[index] != attacks {
                fail = true;
                break;
            }
            used_by[index] = attacks;
        }

        if !fail {
            return attempt;
        }
    }
}


const BISHOP_MAGICS_RAW: [(u64, u32); 64] = [
   (0x0000404040404040,  33104 ),
   (0x0000a060401007fc,   4094 ),
   (0x0000401020200000,  24764 ),
   (0x0000806004000000,  13882 ),
   (0x0000440200000000,  23090 ),
   (0x0000080100800000,  32640 ),
   (0x0000104104004000,  11558 ),
   (0x0000020020820080,  32912 ),
   (0x0000040100202004,  13674 ),
   (0x0000020080200802,   6109 ),
   (0x0000010040080200,  26494 ),
   (0x0000008060040000,  17919 ),
   (0x0000004402000000,  25757 ),
   (0x00000021c100b200,  17338 ),
   (0x0000000400410080,  16983 ),
   (0x000003f7f05fffc0,  16659 ),
   (0x0004228040808010,  13610 ),
   (0x0000200040404040,   2224 ),
   (0x0000400080808080,  60405 ),
   (0x0000200200801000,   7983 ),
   (0x0000240080840000,     17 ),
   (0x000018000c03fff8,  34321 ),
   (0x00000a5840208020,  33216 ),
   (0x0000058408404010,  17127 ),
   (0x0002022000408020,   6397 ),
   (0x0000402000408080,  22169 ),
   (0x0000804000810100,  42727 ),
   (0x000100403c0403ff,    155 ),
   (0x00078402a8802000,   8601 ),
   (0x0000101000804400,  21101 ),
   (0x0000080800104100,  29885 ),
   (0x0000400480101008,  29340 ),
   (0x0001010102004040,  19785 ),
   (0x0000808090402020,  12258 ),
   (0x0007fefe08810010,  50451 ),
   (0x0003ff0f833fc080,   1712 ),
   (0x007fe08019003042,  78475 ),
   (0x0000202040008040,   7855 ),
   (0x0001004008381008,  13642 ),
   (0x0000802003700808,   8156 ),
   (0x0000208200400080,   4348 ),
   (0x0000104100200040,  28794 ),
   (0x0003ffdf7f833fc0,  22578 ),
   (0x0000008840450020,  50315 ),
   (0x0000020040100100,  85452 ),
   (0x007fffdd80140028,  32816 ),
   (0x0000202020200040,  13930 ),
   (0x0001004010039004,  17967 ),
   (0x0000040041008000,  33200 ),
   (0x0003ffefe0c02200,  32456 ),
   (0x0000001010806000,   7762 ),
   (0x0000000008403000,   7794 ),
   (0x0000000100202000,  22761 ),
   (0x0000040100200800,  14918 ),
   (0x0000404040404000,  11620 ),
   (0x00006020601803f4,  15925 ),
   (0x0003ffdfdfc28048,  32528 ),
   (0x0000000820820020,  12196 ),
   (0x0000000010108060,  32720 ),
   (0x0000000000084030,  26781 ),
   (0x0000000001002020,  19817 ),
   (0x0000000040408020,  24732 ),
   (0x0000004040404040,  25468 ),
   (0x0000404040404040,  10186 ),
];

const ROOK_MAGICS_RAW: [(u64, u32); 64] = [
   (0x00280077ffebfffe,  41305 ),
   (0x2004010201097fff,  14326 ),
   (0x0010020010053fff,  24477 ),
   (0x0030002ff71ffffa,   8223 ),
   (0x7fd00441ffffd003,  49795 ),
   (0x004001d9e03ffff7,  60546 ),
   (0x004000888847ffff,  28543 ),
   (0x006800fbff75fffd,  79282 ),
   (0x000028010113ffff,   6457 ),
   (0x0020040201fcffff,   4125 ),
   (0x007fe80042ffffe8,  81021 ),
   (0x00001800217fffe8,  42341 ),
   (0x00001800073fffe8,  14139 ),
   (0x007fe8009effffe8,  19465 ),
   (0x00001800602fffe8,   9514 ),
   (0x000030002fffffa0,  71090 ),
   (0x00300018010bffff,  75419 ),
   (0x0003000c0085fffb,  33476 ),
   (0x0004000802010008,  27117 ),
   (0x0002002004002002,  85964 ),
   (0x0002002020010002,  54915 ),
   (0x0001002020008001,  36544 ),
   (0x0000004040008001,  71854 ),
   (0x0000802000200040,  37996 ),
   (0x0040200010080010,  30398 ),
   (0x0000080010040010,  55939 ),
   (0x0004010008020008,  53891 ),
   (0x0000040020200200,  56963 ),
   (0x0000010020020020,  77451 ),
   (0x0000010020200080,  12319 ),
   (0x0000008020200040,  88500 ),
   (0x0000200020004081,  51405 ),
   (0x00fffd1800300030,  72878 ),
   (0x007fff7fbfd40020,    676 ),
   (0x003fffbd00180018,  83122 ),
   (0x001fffde80180018,  22206 ),
   (0x000fffe0bfe80018,  75186 ),
   (0x0001000080202001,    681 ),
   (0x0003fffbff980180,  36453 ),
   (0x0001fffdff9000e0,  20369 ),
   (0x00fffeebfeffd800,   1981 ),
   (0x007ffff7ffc01400,  13343 ),
   (0x0000408104200204,  10650 ),
   (0x001ffff01fc03000,  57987 ),
   (0x000fffe7f8bfe800,  26302 ),
   (0x0000008001002020,  58357 ),
   (0x0003fff85fffa804,  40546 ),
   (0x0001fffd75ffa802,      0 ),
   (0x00ffffec00280028,  14967 ),
   (0x007fff75ff7fbfd8,  80361 ),
   (0x003fff863fbf7fd8,  40905 ),
   (0x001fffbfdfd7ffd8,  58347 ),
   (0x000ffff810280028,  20381 ),
   (0x0007ffd7f7feffd8,  81868 ),
   (0x0003fffc0c480048,  59381 ),
   (0x0001ffffafd7ffd8,  84404 ),
   (0x00ffffe4ffdfa3ba,  45811 ),
   (0x007fffef7ff3d3da,  62898 ),
   (0x003fffbfdfeff7fa,  45796 ),
   (0x001fffeff7fbfc22,  66994 ),
   (0x0000020408001001,  67204 ),
   (0x0007fffeffff77fd,  32448 ),
   (0x0003ffffbf7dfeec,  62946 ),
   (0x0001ffff9dffa333,  17005),
];