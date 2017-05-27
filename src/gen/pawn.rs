// contains methods for generating pawn moves
use mv_list::MoveList;
use piece::*;
use bb::*;
use board::Board;
use side::{Side, WHITE};
use square;
use super::slider::*;

pub fn pawn_moves<L: MoveList>(board: &Board,
                               capture_mask: BB,
                               push_mask: BB,
                               from_mask: BB,
                               list: &mut L) {
    pawn_pushes(board, push_mask, from_mask, list);
    pawn_captures(board, capture_mask, push_mask, from_mask, list);
}

// white, left  = +7 remove FILE_H
// white, right = +9 remove FILE_A
// black, left  = -9 remove FILE_H
// black, right = -7 remove FILE_A
// maps: side -> capture-direction -> shift amount + overflow mask
pub const PAWN_CAPTURE_FILE_MASKS: [[(usize, BB); 2]; 2] =
    [[(7, NOT_FILE_H), (9, NOT_FILE_A)], [(64 - 9, NOT_FILE_H), (64 - 7, NOT_FILE_A)]];


pub fn pawn_pushes<L: MoveList>(board: &Board, to_mask: BB, from_mask: BB, list: &mut L) {
    // NOTE in the case of EP capture, mask is for the enemy piece taken
    let stm = board.state().stm;
    let piece = PAWN.pc(stm);
    let movers = board.bb_pc(piece) & from_mask;

    if movers == EMPTY {
        return;
    }

    let shift = if stm == WHITE { 8 } else { 64 - 8 };
    let empty_squares = board.bb_empty();

    // Dont add mask here to avoid masking double pushes
    let single_pushes = movers.rot_left(shift as u32) & empty_squares;

    list.add_pawn_pushes(shift, single_pushes & to_mask);

    let mask = if stm == WHITE { ROW_4 } else { ROW_5 };
    let double_pushes = single_pushes.rot_left(shift as u32) & empty_squares & mask;

    // DOUBLE PUSHES
    let double_push_shift = (shift * 2) % 64;
    list.add_pawn_pushes(double_push_shift, double_pushes & to_mask);
}

pub fn pawn_captures<L: MoveList>(board: &Board,
                                  capture_mask: BB,
                                  push_mask: BB,
                                  from_mask: BB,
                                  list: &mut L) {
    let move_mask = capture_mask | push_mask;
    let stm = board.state().stm;
    let piece = PAWN.pc(stm);
    let movers = board.bb_pc(piece) & from_mask;

    if movers == EMPTY {
        return;
    }

    let enemies = board.bb_side(stm.flip());

    if capture_mask != EMPTY {
        // CAPTURES
        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
            let targets = movers.rot_left(shift as u32) & file_mask;
            let occupied_targets = targets & enemies & move_mask;
            list.add_pawn_captures(shift, occupied_targets);
        }
    }

    if board.state().ep_square.is_some() {
        let ep = board.state().ep_square.unwrap();
        // This is rare so worth duplicating work here to avoid doing it above
        for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[stm.to_usize()].iter() {
            // EN-PASSANT CAPTURES
            let targets = movers.rot_left(shift as u32) & file_mask;
            let ep_captures = targets & BB::new(ep);
            for (to, to_bb) in ep_captures.iter() {
                let from = to.rotate_right(shift as square::Internal);

                let capture_sq = from.along_row_with_col(to);
                let capture_sq_bb = BB::new(capture_sq);

                // can only make ep capture if moving to push_mask, or capturing on capture mask
                if ((to_bb & push_mask) | (capture_sq_bb & capture_mask)).any() {
                    // here we need to ensure that there is no discovered check
                    let from_bb = to_bb.rot_right(shift as u32);
                    // This is expensive but very infrequent
                    if !ep_move_discovers_check(from_bb, capture_sq_bb, stm, board) {
                        list.add_pawn_ep_capture(from_bb.bitscan(), ep);
                    }
                }
            }
        }
    }
}

pub fn ep_move_discovers_check(from: BB, to: BB, side: Side, board: &Board) -> bool {
    let occupied = board.bb_occupied() ^ from ^ to;
    let attacker = side.flip();
    let queens = board.bb_pc(QUEEN.pc(attacker));
    let rooks = board.bb_pc(ROOK.pc(attacker));
    let non_diag_attackers = queens | rooks;

    let king_sq = board.bb_pc(KING.pc(side)).bitscan();

    rank_attacks_from_sq(king_sq, occupied) & non_diag_attackers != EMPTY
}

#[cfg(test)]
mod test {
    use gen::util::{assert_list_includes_moves, assert_list_excludes_moves};
    use super::*;
    use mv_list::MoveVec;

    #[test]
    fn captures() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/8/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["a6xb7"]);

        let board = &Board::from_fen("rnbqkbnr/pppppppp/7P/8/8/8/P1PPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_list_includes_moves(&list, &["g7xh6"]);
    }

    #[test]
    fn ep_captures() {
        let board = &Board::from_fen("rnbqkbnr/8/8/Pp6/8/8/8/RNBQKBNR w - b6").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 2);
        assert_list_includes_moves(&list, &["a5xb6e.p.", "a5a6"]);
    }

    #[test]
    fn illegal_ep_captures() {
        // test that ep captures that lead to discovered check due to moving
        // two pieces are not allowed
        //   ABCDEFGH
        // 8|....k...|8
        // 7|........|7
        // 6|....X...|6 -> e6 en-passant square
        // 5|r..Pp..K|5 -> discovered check due to EP
        // 4|........|4
        // 3|........|3
        // 2|........|2
        // 1|........|1
        //   ABCDEFGH
        let board = &Board::from_fen("4k3/8/8/r2Pp2K/8/8/8/8 w - e6").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_list_excludes_moves(&list, &["d5xe6e.p."]);
        assert_list_includes_moves(&list, &["d5d6"]);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn single_pushes() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        println!("{}", list);
        assert_list_includes_moves(&list,
                                   &["a2a3", "b2b3", "c2c3", "d2d3", "e2e3", "f2f3", "g2g3",
                                     "h2h3"]);

        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();
        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_list_includes_moves(&list,
                                   &["a7a6", "b7b6", "c7c6", "d7d6", "e7e6", "f7f6", "g7g6",
                                     "h7h6"]);
    }

    #[test]
    fn double_pushes() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();

        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);

        assert_eq!(list.len(), 16);
        assert_list_includes_moves(&list,
                                   &["a2a4", "b2b4", "c2c4", "d2d4", "e2e4", "f2f4", "g2g4",
                                     "h2h4"]);

        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();

        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 16);
        assert_list_includes_moves(&list,
                                   &["a7a5", "b7b5", "c7c5", "d7d5", "e7e5", "f7f5", "g7g5",
                                     "h7h5"]);
    }

    #[test]
    fn promotions() {
        let board = &Board::from_fen("R1Rqkbnr/PPpppppp/8/8/8/8/8/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();

        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b7b8=N", "b7b8=B", "b7b8=R", "b7b8=Q"]);

        let board = &Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPpPPPP/RN3BNR b").unwrap();
        let mut list = MoveVec::new();

        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["d2d1=N", "d2d1=B", "d2d1=R", "d2d1=Q"]);
    }

    #[test]
    fn promotion_captures() {
        let board = &Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPpPPPP/RNBBNR2 b").unwrap();
        let mut list = MoveVec::new();

        pawn_moves::<MoveVec>(board, !EMPTY, !EMPTY, !EMPTY, &mut list);
        println!("{}", &list);
        assert_eq!(list.len(), 8);
        assert_list_includes_moves(&list,
                                   &["d2xc1=N", "d2xc1=B", "d2xc1=R", "d2xc1=Q", "d2xe1=N",
                                     "d2xe1=B", "d2xe1=R", "d2xe1=Q"]);
    }
}
