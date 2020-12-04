mod ray_hyperbola;
mod ray_kogge_stone;
mod ray_naive;
mod ray_subtract;

#[cfg(target_feature = "bmi2")]
mod ray_bmi2;

#[cfg(not(target_feature = "bmi2"))]
mod ray_magic;

#[cfg(test)]
mod testing;

pub mod consts;

pub use self::ray_hyperbola::rank_attacks_from_sq;
pub use self::ray_kogge_stone::diag_pin_rays_including_attackers;
pub use self::ray_kogge_stone::non_diag_pin_rays_including_attackers;
pub use self::ray_kogge_stone::pinned_pieces;
pub use self::ray_kogge_stone::{bishop_attacks, rook_attacks};
pub use self::ray_kogge_stone::{pin_ray_diag, pin_ray_non_diag};

#[cfg(target_feature = "bmi2")]
pub use self::ray_bmi2::{bishop_attacks_from_sq, rook_attacks_from_sq};

#[cfg(not(target_feature = "bmi2"))]
pub use self::ray_magic::{bishop_attacks_from_sq, rook_attacks_from_sq};

use super::consts::lines_along;
use bb::BB;
use mv_list::MoveAdder;
use piece::{BISHOP, QUEEN, ROOK};
use position::Position;
use square::Square;

pub fn slider_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    pinned_mask: BB,
    king_sq: Square,
    list: &mut L,
) {
    let stm = position.state().stm;
    let occupied = position.bb_occupied();
    let queens = position.bb_pc(QUEEN.pc(stm));
    let rooks = position.bb_pc(ROOK.pc(stm));
    let bishops = position.bb_pc(BISHOP.pc(stm));
    let diag_attackers = queens | bishops;
    let non_diag_attackers = queens | rooks;

    for (from, _) in (non_diag_attackers & !pinned_mask).iter() {
        debug_assert!(position.at(from) == ROOK.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let targets = rook_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }

    for (from, _) in (non_diag_attackers & pinned_mask).iter() {
        debug_assert!(position.at(from) == ROOK.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let ray_mask = lines_along(from, king_sq);
        let targets = rook_attacks_from_sq(from, occupied) & ray_mask;
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }

    for (from, _) in (diag_attackers & !pinned_mask).iter() {
        debug_assert!(position.at(from) == BISHOP.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let targets = bishop_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }

    for (from, _) in (diag_attackers & pinned_mask).iter() {
        debug_assert!(position.at(from) == BISHOP.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let ray_mask = lines_along(from, king_sq);
        let targets = bishop_attacks_from_sq(from, occupied) & ray_mask;
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }
}

pub fn slider_captures<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    pinned_mask: BB,
    king_sq: Square,
    list: &mut L,
) {
    let stm = position.state().stm;
    let occupied = position.bb_occupied();
    let queens = position.bb_pc(QUEEN.pc(stm));
    let rooks = position.bb_pc(ROOK.pc(stm));
    let bishops = position.bb_pc(BISHOP.pc(stm));
    let diag_attackers = queens | bishops;
    let non_diag_attackers = queens | rooks;

    for (from, _) in (non_diag_attackers & !pinned_mask).iter() {
        debug_assert!(position.at(from) == ROOK.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let targets = rook_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
    }

    for (from, _) in (non_diag_attackers & pinned_mask).iter() {
        debug_assert!(position.at(from) == ROOK.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let ray_mask = lines_along(from, king_sq);
        let targets = rook_attacks_from_sq(from, occupied) & ray_mask;
        list.add_captures(from, targets & capture_mask);
    }

    for (from, _) in (diag_attackers & !pinned_mask).iter() {
        debug_assert!(position.at(from) == BISHOP.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let targets = bishop_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
    }

    for (from, _) in (diag_attackers & pinned_mask).iter() {
        debug_assert!(position.at(from) == BISHOP.pc(stm) || position.at(from) == QUEEN.pc(stm));
        let ray_mask = lines_along(from, king_sq);
        let targets = bishop_attacks_from_sq(from, occupied) & ray_mask;
        list.add_captures(from, targets & capture_mask);
    }
}

#[allow(dead_code)]
pub fn non_diag_slider_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    from_mask: BB,
    list: &mut L,
) {
    let stm = position.state().stm;
    let occupied = position.bb_occupied();
    let queens = position.bb_pc(QUEEN.pc(stm));
    let rooks = position.bb_pc(ROOK.pc(stm));
    let non_diag_attackers = (queens | rooks) & from_mask;

    for (from, _) in non_diag_attackers.iter() {
        let targets = rook_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }
}

#[allow(dead_code)]
pub fn diag_slider_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    from_mask: BB,
    list: &mut L,
) {
    let stm = position.state().stm;
    let occupied = position.bb_occupied();
    let queens = position.bb_pc(QUEEN.pc(stm));
    let bishops = position.bb_pc(BISHOP.pc(stm));
    let diag_attackers = (queens | bishops) & from_mask;

    for (from, _) in diag_attackers.iter() {
        let targets = bishop_attacks_from_sq(from, occupied);
        list.add_captures(from, targets & capture_mask);
        list.add_non_captures(from, targets & push_mask);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bb::EMPTY;
    use gen::util::assert_list_includes_moves;
    use mv_list::MoveVec;

    #[test]
    fn test_rook_moves() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNB1KBNR w").unwrap();
        let mut list = MoveVec::new();

        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        slider_moves::<MoveVec>(
            position,
            capture_mask,
            push_mask,
            EMPTY,
            Square(1),
            &mut list,
        );
        assert_list_includes_moves(&list, &["a1xa7", "a1a2", "a1a3", "a1a4", "a1a5", "a1a6"]);
    }

    #[test]
    fn test_bishop_moves() {
        let position = &Position::from_fen("rnbqkbnr/4pppp/8/5P2/8/8/8/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();

        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        slider_moves::<MoveVec>(
            position,
            capture_mask,
            push_mask,
            EMPTY,
            Square(1),
            &mut list,
        );
        assert_list_includes_moves(&list, &["c8xf5", "c8a6", "c8e6", "c8b7", "c8d7"]);
    }

    #[test]
    fn test_queen_moves() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPP1PPPP/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();

        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        slider_moves::<MoveVec>(
            position,
            capture_mask,
            push_mask,
            EMPTY,
            Square(1),
            &mut list,
        );
        assert_list_includes_moves(&list, &["d1xd7", "d1d2", "d1d3", "d1d4", "d1d5", "d1d6"]);
    }
}
