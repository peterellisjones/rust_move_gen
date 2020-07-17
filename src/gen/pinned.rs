use bb::*;
use gen::pawn::PAWN_CAPTURE_FILE_MASKS;
use mv_list::MoveList;
use piece::*;
use position::Position;
use side::{Side, WHITE};
use square::Square;

// Generates pawn moves along pin rays
// NOTE: this is quigte expensive unfortunately
pub fn pawn_pin_ray_moves<L: MoveList>(
    position: &Position,
    king_sq: Square,
    pinned: BB,
    pinners: BB,
    stm: Side,
    list: &mut L,
) {
    let empty_squares = position.bb_empty();
    let piece = PAWN.pc(stm);
    let movers = position.bb_pc(piece) & pinned;

    let push_shift = if stm == WHITE { 8 } else { 64 - 8 };
    let push_mask = if stm == WHITE { ROW_4 } else { ROW_5 };

    for (pawn_sq, pawn) in movers.iter() {
        // if king is on the same file, the pawn can "push"
        // as the pin is a north-south pin
        if king_sq.same_file(pawn_sq) {
            let single_pushes = pawn.rot_left(push_shift as u32) & empty_squares;
            list.add_pawn_pushes(push_shift, single_pushes);
            let double_pushes =
                single_pushes.rot_left(push_shift as u32) & empty_squares & push_mask;
            let double_push_shift = (push_shift * 2) % 64;
            list.add_pawn_pushes(double_push_shift, double_pushes);
        }

        // iterate over the two possible capture directions..
        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
            // captures can only happen if the pinned piece take its pinner
            // There could be multiple pinners, but since apawn can only capture 1 square
            // it can only ever take the pinner that is pinning it
            let targets = pinned.rot_left(shift as u32) & file_mask & pinners;
            list.add_pawn_captures(shift, targets);

            // no need to consider ep-capture since a pawn can never pin another piece
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gen::util::assert_list_includes_moves;
    use mv_list::MoveVec;
    use position::Position;
    use square::*;

    #[test]
    fn test_pawn_pin_ray_moves() {
        // test pawn capture along pin ray
        let position =
            Position::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/1PB5/8/PP2N1PP/RNB1K2R w KQ - 3 9")
                .unwrap();

        let mut list = MoveVec::new();
        pawn_pin_ray_moves(&position, E1, BB::new(B4), BB::new(A5), WHITE, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["b4xa5"]);
    }

    #[test]
    fn test_pawn_pin_ray_moves_2() {
        // test pawn move along pin ray
        let position =
            Position::from_fen("rnb2k1r/pp1Pbppp/2p5/4q3/8/8/PP2P1PP/RNB1KNBR w KQ - 3 9").unwrap();

        let mut list = MoveVec::new();
        pawn_pin_ray_moves(&position, E1, BB::new(E2), BB::new(E5), WHITE, &mut list);
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
            E1,
            BB::new(B4) | BB::new(E3),
            BB::new(A5) | BB::new(E5),
            WHITE,
            &mut list,
        );
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["b4xa5"]);
    }
}
