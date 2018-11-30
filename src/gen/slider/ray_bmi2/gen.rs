use super::super::ray_naive::*;
use bb::*;
use square::*;
use std::arch::x86_64::{_pdep_u64, _pext_u64};

#[allow(dead_code)]
pub fn generate_offsets() {
  let mut all_attacks = [0u64; 200000];

  let mut offset = 0;

  println!("use bb::BB;");
  println!("");
  println!("pub struct Offset {{");
  println!("    pub inner_mask: BB,");
  println!("    pub outer_mask: BB,");
  println!("    pub base_offset: u32,");
  println!("}}");

  println!("pub const BISHOP_OFFSETS: [Offset; 64] = [");
  for i in 0..64 {
    let sq = Square(i);
    let inner_mask = bishop_inner_mask(sq);
    let outer_mask = bishop_outer_mask(sq);
    let occupancy_attack_map =
      generate_occupancy_attack_map(sq, bishop_attacks_from_sq, inner_mask);
    let base_offset = offset;

    for occupancy in 0..(1 << inner_mask.pop_count()) {
      let (occupied, attacks) = occupancy_attack_map[occupancy];

      // verify that occupied BB can be PEXTd to occupancy
      assert_eq!(
        unsafe { _pext_u64(occupied.0, inner_mask.0) },
        occupancy as u64
      );

      all_attacks[offset] = unsafe { _pext_u64(attacks.0, outer_mask.0) };

      // verify that attacks can be PDEPd to attacks BB
      assert_eq!(
        BB(unsafe { _pdep_u64(all_attacks[offset] as u64, outer_mask.0) }),
        attacks
      );

      offset += 1;
    }

    println!(
      "\tOffset {{ inner_mask: BB({}), outer_mask: BB({}), base_offset: {} }},",
      inner_mask.0, outer_mask.0, base_offset,
    );
  }
  println!("];\n");

  println!("pub const ROOK_OFFSETS: [Offset; 64] = [");
  for i in 0..64 {
    let sq = Square(i);
    let inner_mask = rook_inner_mask(sq);
    let outer_mask = rook_outer_mask(sq);
    let occupancy_attack_map = generate_occupancy_attack_map(sq, rook_attacks_from_sq, inner_mask);
    let base_offset = offset;

    for occupancy in 0..(1 << inner_mask.pop_count()) {
      let (_, attacks) = occupancy_attack_map[occupancy];

      all_attacks[offset] = unsafe { _pext_u64(attacks.0, outer_mask.0) };

      offset += 1;
    }

    println!(
      "\tOffset {{ inner_mask: BB({}), outer_mask: BB({}), base_offset: {} }},",
      inner_mask.0, outer_mask.0, base_offset,
    );
  }
  println!("];\n");

  println!("pub const SHARED_ATTACK_INDICES: [u16; 107648] = [");
  for i in 0..offset {
    println!("\t{},", all_attacks[i]);
  }
  println!("];\n");
}

fn rook_outer_mask(sq: Square) -> BB {
  let horizontal_mask = (ROW_1 << sq.rowx8());
  let vertical_mask = (FILE_A << sq.col());
  let sq_bb = BB(1 << sq.0);

  (horizontal_mask | vertical_mask) & (!sq_bb)
}

fn bishop_outer_mask(sq: Square) -> BB {
  DIAGONALS[sq.to_usize()] | ANTI_DIAGONALS[sq.to_usize()]
}

fn rook_inner_mask(sq: Square) -> BB {
  let horizontal_mask = (ROW_1 << sq.rowx8()) & !(FILE_A | FILE_H);
  let vertical_mask = (FILE_A << sq.col()) & !(ROW_1 | ROW_8);
  let sq_bb = BB(1 << sq.0);

  (horizontal_mask | vertical_mask) & (!sq_bb)
}

fn bishop_inner_mask(sq: Square) -> BB {
  let xray = DIAGONALS[sq.to_usize()] | ANTI_DIAGONALS[sq.to_usize()];

  xray & !(ROW_1 | ROW_8 | FILE_A | FILE_H)
}

fn generate_occupancy_attack_map<F: Fn(Square, BB) -> BB>(
  sq: Square,
  attack_func: F,
  mask: BB,
) -> [(BB, BB); 4096] {
  let mut map = [(BB(0), BB(0)); 4096];

  for occupancy in 0..(1 << mask.pop_count()) {
    let mut occupied = BB(0);

    for (idx, mask_sq) in mask.square_list().iter().enumerate() {
      if (occupancy & (1 << idx)) != 0 {
        occupied |= BB(1) << mask_sq.to_usize();
      }
    }

    let attacks = attack_func(sq, occupied);
    map[occupancy] = (occupied, attacks);
  }

  map
}

pub const DIAGONALS: [BB; 64] = [
  BB(0),
  BB(256),
  BB(66048),
  BB(16909312),
  BB(4328785920),
  BB(1108169199616),
  BB(283691315109888),
  BB(72624976668147712),
  BB(2),
  BB(65540),
  BB(16908296),
  BB(4328783888),
  BB(1108169195552),
  BB(283691315101760),
  BB(72624976668131456),
  BB(145249953336262656),
  BB(516),
  BB(16778248),
  BB(4328523792),
  BB(1108168675360),
  BB(283691314061376),
  BB(72624976666050688),
  BB(145249953332101120),
  BB(290499906664136704),
  BB(132104),
  BB(4295231504),
  BB(1108102090784),
  BB(283691180892224),
  BB(72624976399712384),
  BB(145249952799424512),
  BB(290499905598783488),
  BB(580999811180789760),
  BB(33818640),
  BB(1099579265056),
  BB(283674135240768),
  BB(72624942308409472),
  BB(145249884616818688),
  BB(290499769233571840),
  BB(580999538450366464),
  BB(1161999072605765632),
  BB(8657571872),
  BB(281492291854400),
  BB(72620578621636736),
  BB(145241157243273216),
  BB(290482314486480896),
  BB(580964628956184576),
  BB(1161929253617401856),
  BB(2323857407723175936),
  BB(2216338399296),
  BB(72062026714726528),
  BB(144124053429452800),
  BB(288248106858840064),
  BB(576496213700902912),
  BB(1152992423106838528),
  BB(2305983746702049280),
  BB(4611686018427387904),
  BB(567382630219904),
  BB(1134765260439552),
  BB(2269530520813568),
  BB(4539061024849920),
  BB(9078117754732544),
  BB(18155135997837312),
  BB(36028797018963968),
  BB(0),
];

pub const ANTI_DIAGONALS: [BB; 64] = [
  BB(9241421688590303744),
  BB(36099303471055872),
  BB(141012904183808),
  BB(550831656960),
  BB(2151686144),
  BB(8404992),
  BB(32768),
  BB(0),
  BB(4620710844295151616),
  BB(9241421688590303233),
  BB(36099303471054850),
  BB(141012904181764),
  BB(550831652872),
  BB(2151677968),
  BB(8388640),
  BB(64),
  BB(2310355422147510272),
  BB(4620710844295020800),
  BB(9241421688590041601),
  BB(36099303470531586),
  BB(141012903135236),
  BB(550829559816),
  BB(2147491856),
  BB(16416),
  BB(1155177711056977920),
  BB(2310355422114021376),
  BB(4620710844228043008),
  BB(9241421688456086017),
  BB(36099303202620418),
  BB(141012367312900),
  BB(549757915144),
  BB(4202512),
  BB(577588851233521664),
  BB(1155177702483820544),
  BB(2310355404967706624),
  BB(4620710809935413504),
  BB(9241421619870827009),
  BB(36099166032102402),
  BB(140738026276868),
  BB(1075843080),
  BB(288793326105133056),
  BB(577586656505233408),
  BB(1155173313027244032),
  BB(2310346626054553600),
  BB(4620693252109107456),
  BB(9241386504218214913),
  BB(36028934726878210),
  BB(275415828484),
  BB(144115188075855872),
  BB(288231475663339520),
  BB(576462955621646336),
  BB(1152925911260069888),
  BB(2305851822520205312),
  BB(4611703645040410880),
  BB(9223407290080821761),
  BB(70506452091906),
  BB(0),
  BB(281474976710656),
  BB(564049465049088),
  BB(1128103225065472),
  BB(2256206466908160),
  BB(4512412933881856),
  BB(9024825867763968),
  BB(18049651735527937),
];

#[cfg(test)]
mod test {
  use super::*;
  use test;

  #[test]
  fn test_generate_offsets() {
    // uncomment to dump BMI offset table to `cargo test` output
    // generate_offsets();
    // assert!(false);
  }
}
