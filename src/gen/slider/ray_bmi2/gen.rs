use super::super::ray_naive::*;
use bb::*;
use square::*;
use std::arch::x86_64::{_pdep_u64, _pext_u64};

#[allow(dead_code)]
pub fn generate_offsets() {
  let mut all_attacks = [0u64; 200000];

  let mut offset = 0;

  println!("use bb::BB;");
  println!();
  println!("pub struct Offset {{");
  println!("    pub inner_mask: BB,");
  println!("    pub outer_mask: BB,");
  println!("    pub base_offset: u32,");
  println!("}}");

  println!("pub const BISHOP_OFFSETS: [Offset; 64] = [");
  for i in 0..64 {
    let sq = Square(i);
    let inner_mask = bishop_inner_mask(sq);
    let outer_mask = sq.bishop_rays();
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
    let outer_mask = sq.rook_rays();
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
  let horizontal_mask = ROW_1 << sq.rowx8();
  let vertical_mask = FILE_A << sq.col();
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
