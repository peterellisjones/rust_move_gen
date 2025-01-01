use crate::bb::*;
use crate::generation::pawn::PAWN_CAPTURE_FILE_MASKS;
use crate::mv_list::MoveAdder;
use crate::piece::*;
use crate::position::Position;
use crate::side::{Side, WHITE};
use crate::square::{Square, SquareInternal};

// Generates pawn moves along pin rays
pub fn pawn_pin_ray_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    king_sq: Square,
    pinned: BB,
    stm: Side,
    list: &mut L,
) {
    let piece = PAWN.pc(stm);
    let movers = position.bb_pc(piece) & pinned;

    // exit early if no pinned pawns
    if movers.none() {
        return;
    }

    let push_shift = if stm == WHITE { 8 } else { 64 - 8 };
    let double_push_mask = push_mask & if stm == WHITE { ROW_4 } else { ROW_5 };

    let can_push = movers & king_sq.file_mask();
    let king_diags = king_sq.bishop_rays();
    let can_capture = movers & king_diags;

    // For pinned pawns, only possible moves are those along the king file
    for (_, pawn) in can_push.iter() {
        let single_pushes = pawn.rot_left(push_shift as u32) & push_mask;
        list.add_pawn_pushes(push_shift, single_pushes);
        let double_pushes = single_pushes.rot_left(push_shift as u32) & double_push_mask;
        let double_push_shift = (push_shift * 2) % 64;
        list.add_pawn_pushes(double_push_shift, double_pushes);
    }

    for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
        let targets = can_capture.rot_left(shift as u32) & file_mask & capture_mask & king_diags;

        list.add_pawn_captures(shift, targets);
    }

    if position.state().ep_square.is_some() {
        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
            let targets = can_capture.rot_left(shift as u32) & file_mask;

            let ep = position.state().ep_square.unwrap();
            let ep_captures = targets & BB::new(ep) & king_diags;

            for (to, to_bb) in ep_captures.iter() {
                let from = to.rotate_right(shift as SquareInternal);

                let capture_sq = from.along_row_with_col(to);
                let capture_sq_bb = BB::new(capture_sq);

                // can only make ep capture if moving along king_diags, or capturing on capture mask
                if ((to_bb & king_diags) | (capture_sq_bb & capture_mask)).any() {
                    let from_bb = to_bb.rot_right(shift as u32);
                    list.add_pawn_ep_capture(from_bb.bitscan(), ep);
                }
            }
        }
    }
}

pub fn pawn_pin_ray_captures<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    king_sq: Square,
    pinned: BB,
    stm: Side,
    list: &mut L,
) {
    let piece = PAWN.pc(stm);
    let movers = position.bb_pc(piece) & pinned;

    // exit early if no pinned pawns
    if movers.none() {
        return;
    }

    let king_diags = king_sq.bishop_rays();
    let can_capture = movers & king_diags;

    for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
        let targets = can_capture.rot_left(shift as u32) & file_mask & capture_mask & king_diags;

        list.add_pawn_captures(shift, targets);
    }

    if position.state().ep_square.is_some() {
        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
            let targets = can_capture.rot_left(shift as u32) & file_mask;

            let ep = position.state().ep_square.unwrap();
            let ep_captures = targets & BB::new(ep) & king_diags;

            for (to, to_bb) in ep_captures.iter() {
                let from = to.rotate_right(shift as SquareInternal);

                let capture_sq = from.along_row_with_col(to);
                let capture_sq_bb = BB::new(capture_sq);

                // can only make ep capture if moving along king_diags, or capturing on capture mask
                if ((to_bb & king_diags) | (capture_sq_bb & capture_mask)).any() {
                    let from_bb = to_bb.rot_right(shift as u32);
                    list.add_pawn_ep_capture(from_bb.bitscan(), ep);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::generation::util::assert_list_includes_moves;
    use crate::mv_list::MoveVec;
    use crate::position::Position;
    use crate::square::*;

    #[test]
    fn test_pawn_pin_ray_moves() {
        // test pawn capture along pin ray
        let position =
            Position::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/1PB5/8/PP2N1PP/RNB1K2R w KQ - 3 9")
                .unwrap();

        let mut list = MoveVec::new();
        pawn_pin_ray_moves(
            &position,
            BB::new(A5),
            !EMPTY,
            E1,
            BB::new(B4),
            WHITE,
            &mut list,
        );
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["b4xa5"]);
    }

    #[test]
    fn test_pawn_pin_ray_moves_2() {
        // test pawn move along pin ray
        let position =
            Position::from_fen("rnb2k1r/pp1Pbppp/2p5/4q3/8/8/PP2P1PP/RNB1KNBR w KQ - 3 9").unwrap();

        let mut list = MoveVec::new();
        pawn_pin_ray_moves(
            &position,
            BB::new(E5),
            !EMPTY,
            E1,
            BB::new(E2),
            WHITE,
            &mut list,
        );
        assert_eq!(list.len(), 2);
        assert_list_includes_moves(&list, &["e2e3", "e2e4"]);
    }

    #[test]
    fn test_pawn_pin_ray_moves_3() {
        // test pawn capture along pin ray when multiple pinners
        let position =
            Position::from_fen("r1b2k2/pp1Pbppp/2p5/q1n1r3/1PB5/4R3/P4PPP/RNB1K3 w Q - 3 9")
                .unwrap();

        let mut list = MoveVec::new();
        pawn_pin_ray_moves(
            &position,
            BB::new(A5) | BB::new(E5),
            !EMPTY,
            E1,
            BB::new(B4) | BB::new(E3),
            WHITE,
            &mut list,
        );
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["b4xa5"]);
    }
}
