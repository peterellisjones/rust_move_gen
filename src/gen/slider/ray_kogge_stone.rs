use bb::*;

#[cfg(target_feature = "sse3")]
use dbb::*;

/// calculates the bitboard of pinned pieces
#[inline]
pub fn pinned_pieces(king: BB, empty: BB, enemy_diag: BB, enemy_non_diag: BB) -> BB {
    diag_pinned_pieces(king, empty, enemy_diag) |
    non_diag_pinned_pieces(king, empty, enemy_non_diag)
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
fn diag_pinned_pieces(king: BB, empty: BB, enemy_bishops: BB) -> BB {
    let north_west = king.north_west_attacks(empty) & enemy_bishops.south_east_attacks(empty);
    let south_west = king.south_west_attacks(empty) & enemy_bishops.north_east_attacks(empty);
    let north_east = king.north_east_attacks(empty) & enemy_bishops.south_west_attacks(empty);
    let south_east = king.south_east_attacks(empty) & enemy_bishops.north_west_attacks(empty);

    north_east | north_west | south_east | south_west
}

#[cfg(target_feature = "sse3")]
#[inline]
fn diag_pinned_pieces(king_bb: BB, empty_bb: BB, enemy_bishops_bb: BB) -> BB {
    let king_and_enemy_bishops = DBB::new(king_bb, enemy_bishops_bb);
    let enemy_bishop_and_king = DBB::new(enemy_bishops_bb, king_bb);
    let empty = DBB::splat(empty_bb);

    let nw_and_se = king_and_enemy_bishops.north_west_attacks(empty) &
                    enemy_bishop_and_king.south_east_attacks(empty);
    let ne_and_sw = king_and_enemy_bishops.north_east_attacks(empty) &
                    enemy_bishop_and_king.south_west_attacks(empty);

    let (north, south) = (nw_and_se | ne_and_sw).extract();

    north | south
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
fn non_diag_pinned_pieces(king: BB, empty: BB, enemy_rooks: BB) -> BB {
    let north = king.north_attacks(empty) & enemy_rooks.south_attacks(empty);
    let south = king.south_attacks(empty) & enemy_rooks.north_attacks(empty);
    let east = king.east_attacks(empty) & enemy_rooks.west_attacks(empty);
    let west = king.west_attacks(empty) & enemy_rooks.east_attacks(empty);

    north | south | east | west
}

#[cfg(target_feature = "sse3")]
#[inline]
fn non_diag_pinned_pieces(king_bb: BB, empty_bb: BB, enemy_rooks: BB) -> BB {
    let king_and_enemy_rooks = DBB::new(king_bb, enemy_rooks);
    let enemy_rooks_and_king = DBB::new(enemy_rooks, king_bb);
    let empty = DBB::splat(empty_bb);

    let n_and_s = king_and_enemy_rooks.north_attacks(empty) &
                  enemy_rooks_and_king.south_attacks(empty);
    let e_and_w = king_and_enemy_rooks.east_attacks(empty) &
                  enemy_rooks_and_king.west_attacks(empty);

    let (pos, neg) = (n_and_s | e_and_w).extract();

    pos | neg
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn diag_pin_rays_including_attackers(source: BB, empty: BB, enemy_diag_pieces: BB) -> (BB, BB) {
    let north_west = source.north_west_attacks(empty) &
                     enemy_diag_pieces.occluded_south_east_fill(empty);
    let south_west = source.south_west_attacks(empty) &
                     enemy_diag_pieces.occluded_north_east_fill(empty);
    let north_east = source.north_east_attacks(empty) &
                     enemy_diag_pieces.occluded_south_west_fill(empty);
    let south_east = source.south_east_attacks(empty) &
                     enemy_diag_pieces.occluded_north_west_fill(empty);

    (north_east | south_west, north_west | south_east)
}

#[cfg(target_feature = "sse3")]
#[inline]
pub fn diag_pin_rays_including_attackers(source_bb: BB,
                                         empty_bb: BB,
                                         enemy_diag_bb: BB)
                                         -> (BB, BB) {
    let source_and_diag = DBB::new(source_bb, enemy_diag_bb);
    let diag_and_source = DBB::new(enemy_diag_bb, source_bb);
    let empty = DBB::splat(empty_bb);

    let ne_and_sw = source_and_diag.occluded_north_east_fill_with_occluders(empty) &
                    diag_and_source.occluded_south_west_fill_with_occluders(empty);

    let nw_and_se = source_and_diag.occluded_north_west_fill_with_occluders(empty) &
                    diag_and_source.occluded_south_east_fill_with_occluders(empty);

    let (north_east, south_west) = ne_and_sw.extract();
    let (north_west, south_east) = nw_and_se.extract();

    (north_east | south_west, north_west | south_east)
}

/// TODO: can we do north-south attacks together by considering the king an attacker?
#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn non_diag_pin_rays_including_attackers(source: BB,
                                             empty: BB,
                                             enemy_non_diag_pieces: BB)
                                             -> (BB, BB) {
    let north = source.north_attacks(empty) & enemy_non_diag_pieces.occluded_south_fill(empty);
    let south = source.south_attacks(empty) & enemy_non_diag_pieces.occluded_north_fill(empty);
    let east = source.east_attacks(empty) & enemy_non_diag_pieces.occluded_west_fill(empty);
    let west = source.west_attacks(empty) & enemy_non_diag_pieces.occluded_east_fill(empty);

    (north | south, east | west)
}

#[cfg(target_feature = "sse3")]
#[inline]
pub fn non_diag_pin_rays_including_attackers(source_bb: BB,
                                             empty_bb: BB,
                                             enemy_non_diag_bb: BB)
                                             -> (BB, BB) {
    let source_and_non_diag = DBB::new(source_bb, enemy_non_diag_bb);
    let non_diag_and_source = DBB::new(enemy_non_diag_bb, source_bb);
    let empty = DBB::splat(empty_bb);

    let n_and_s = source_and_non_diag.occluded_north_fill_with_occluders(empty) &
                  non_diag_and_source.occluded_south_fill_with_occluders(empty);

    let w_and_e = source_and_non_diag.occluded_west_fill_with_occluders(empty) &
                  non_diag_and_source.occluded_east_fill_with_occluders(empty);

    let (north, south) = n_and_s.extract();
    let (west, east) = w_and_e.extract();

    (north | south, east | west)
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn pin_ray_diag(source: BB, empty: BB, enemy_diag_pieces: BB) -> BB {
    let north_west = source.occluded_north_west_fill(empty) &
                     enemy_diag_pieces.occluded_south_east_fill(empty);
    let south_west = source.occluded_south_west_fill(empty) &
                     enemy_diag_pieces.occluded_north_east_fill(empty);
    let north_east = source.occluded_north_east_fill(empty) &
                     enemy_diag_pieces.occluded_south_west_fill(empty);
    let south_east = source.occluded_south_east_fill(empty) &
                     enemy_diag_pieces.occluded_north_west_fill(empty);

    north_east | north_west | south_east | south_west
}

#[cfg(not(target_feature = "sse3"))]
#[inline]
pub fn pin_ray_non_diag(source: BB, empty: BB, enemy_non_diag_pieces: BB) -> BB {
    let north = source.occluded_north_fill(empty) &
                enemy_non_diag_pieces.occluded_south_fill(empty);
    let south = source.occluded_south_fill(empty) &
                enemy_non_diag_pieces.occluded_north_fill(empty);
    let east = source.occluded_east_fill(empty) & enemy_non_diag_pieces.occluded_west_fill(empty);
    let west = source.occluded_west_fill(empty) & enemy_non_diag_pieces.occluded_east_fill(empty);

    north | west | south | east
}

#[cfg(target_feature = "sse3")]
#[inline]
pub fn pin_ray_diag(source_bb: BB, empty_bb: BB, enemy_diag_bb: BB) -> BB {
    let source_and_diag = DBB::new(source_bb, enemy_diag_bb);
    let diag_and_source = DBB::new(enemy_diag_bb, source_bb);
    let empty = DBB::splat(empty_bb);

    let ne_and_sw = source_and_diag.occluded_north_east_fill(empty) &
                    diag_and_source.occluded_south_west_fill(empty);

    let nw_and_se = source_and_diag.occluded_north_west_fill(empty) &
                    diag_and_source.occluded_south_east_fill(empty);

    let (north, south) = (ne_and_sw | nw_and_se).extract();

    north | south
}

#[cfg(target_feature = "sse3")]
#[inline]
pub fn pin_ray_non_diag(source_bb: BB, empty_bb: BB, enemy_non_diag_bb: BB) -> BB {
    let source_and_non_diag = DBB::new(source_bb, enemy_non_diag_bb);
    let non_diag_and_source = DBB::new(enemy_non_diag_bb, source_bb);
    let empty = DBB::splat(empty_bb);

    let n_and_s = source_and_non_diag.occluded_north_fill(empty) &
                  non_diag_and_source.occluded_south_fill(empty);

    let w_and_e = source_and_non_diag.occluded_west_fill(empty) &
                  non_diag_and_source.occluded_east_fill(empty);

    let (pos, neg) = (n_and_s | w_and_e).extract();

    pos | neg
}

#[inline]
pub fn rook_attacks(from: BB, occupied: BB) -> BB {
    let empty = !occupied;
    from.east_attacks(empty) | from.north_attacks(empty) | from.south_attacks(empty) |
    from.west_attacks(empty)
}

#[inline]
pub fn bishop_attacks(from: BB, occupied: BB) -> BB {
    let empty = !occupied;
    from.north_east_attacks(empty) | from.north_west_attacks(empty) |
    from.south_east_attacks(empty) | from.south_west_attacks(empty)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::testing::*;
    use test;

    #[test]
    fn t_bishop_attacks() {
        test_bishop_attacks(bishop_attacks);
    }

    #[test]
    fn t_rook_attacks() {
        test_rook_attacks(rook_attacks);
    }

    #[bench]
    fn bench_rook_attacks(b: &mut test::Bencher) {
        bench_attacks_from_squares(b, rook_attacks);
    }

    #[bench]
    fn bench_bishop_attacks(b: &mut test::Bencher) {
        bench_attacks_from_squares(b, bishop_attacks);
    }
}
