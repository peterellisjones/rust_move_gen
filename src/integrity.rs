use side::*;
use piece::*;
use board::Board;
use square::*;
use castle::*;
use castling_rights::*;

/*
    CHECKS:
    - bitboards and array representation must be consistent
    - piece counts must be legal (ie pawns + rooks <= 10)
    - castling rights only valid if king/rook haven't moved
    - pawns cannot be on rank 1 or 8 for both sides
    - en-passant square must be consistent with pawn locations
    - full move number >= 1

    // TODO: castling rights shuld be invalidated when king in check
*/

#[allow(dead_code)]
pub fn test(board: &Board) -> Option<String> {
    if let Some(err) = test_bitboards(board) {
        return Some(err);
    }

    if let Some(err) = test_piece_counts(board) {
        return Some(err);
    }

    if let Some(err) = test_castling_rights(board) {
        return Some(err);
    }

    if let Some(err) = test_pawn_invalid_rows(board) {
        return Some(err);
    }

    if let Some(err) = test_ep_square(board) {
        return Some(err);
    }

    if board.state().full_move_number < 1 {
        return Some(format!("Error: full move number cannot be less than 1 ({})",
                            board.state().full_move_number));
    }

    None
}

fn test_bitboards(board: &Board) -> Option<String> {
    for &side in &[WHITE, BLACK] {
        let bb = board.bb_side(side);
        for (sq, _) in bb.iter() {
            if board.at(sq).is_some() {
                let pc = board.at(sq).unwrap();
                if pc.side() != side {
                    return Some(format!("Expected {} piece at {} but found {}",
                                        side.to_string(),
                                        sq.to_string(),
                                        board.at(sq).unwrap().to_string()));
                }
            }
        }
    }

    for pc in Piece::iter() {
        let bb = board.bb_pc(pc);
        for (sq, _) in bb.iter() {
            if board.at(sq) != OptionPiece(pc) {
                return Some(format!("Expected {} at {} but found {}",
                                    pc.to_string(),
                                    sq.to_string(),
                                    board.at(sq).unwrap().to_string()));
            }
        }
    }

    None
}

fn test_castling_rights(board: &Board) -> Option<String> {
    let rights = board.state().castling_rights;
    for &(side, king_square) in [(WHITE, E1), (BLACK, E8)].iter() {
        if board.at(king_square) != OptionPiece(KING.pc(side)) {
            if rights.has(CastlingRights::from(QUEEN_SIDE, side)) {
                return Some(format!("Error: {} cannot castle as king has moved",
                                    side.to_string()));
            }
            if rights.has(CastlingRights::from(KING_SIDE, side)) {
                return Some(format!("Error: {} cannot castle as king has moved",
                                    side.to_string()));
            }
        }
    }

    for &(right, rook_square, side) in [(WHITE_QS, A1, WHITE), (BLACK_QS, A8, BLACK)].iter() {
        if rights.has(right) && board.at(rook_square) != OptionPiece(ROOK.pc(side)) {
            return Some(format!("Error: {} cannot castle queen-side as rook has moved",
                                side.to_string()));
        }
    }

    for &(right, rook_square, side) in [(WHITE_KS, H1, WHITE), (BLACK_KS, H8, BLACK)].iter() {
        if rights.has(right) && board.at(rook_square) != OptionPiece(ROOK.pc(side)) {
            return Some(format!("Error: {} cannot castle king-side as rook has moved",
                                side.to_string()));
        }
    }
    None
}

fn test_ep_square(board: &Board) -> Option<String> {
    if board.state().ep_square.is_none() {
        return None;
    }
    let sq = board.state().ep_square.unwrap();

    let stm = board.state().stm;
    if (stm == BLACK && sq.row() != 2) || (stm == WHITE && sq.row() != 5) {
        return Some(format!("Error: en-passant square is {} but side to move is {}",
                            sq,
                            stm.to_string()));
    }

    let target_sq = sq.change_row(if stm == BLACK { 3 } else { 4 });
    let expected_target = PAWN.pc(stm.flip());
    if board.at(target_sq) != OptionPiece(expected_target) {
        return Some(format!("Error: en-passant square is {} but no {} at {}",
                            sq,
                            expected_target.to_string(),
                            target_sq));
    }

    None
}

fn test_pawn_invalid_rows(board: &Board) -> Option<String> {
    for &side in &[WHITE, BLACK] {
        let piece = PAWN.pc(side);
        let bb = board.bb_pc(piece);
        for row in [0, 7].iter() {
            if !bb.row_empty(*row as usize) {
                return Some(format!("Error: {} in invalid position: {:?}",
                                    piece.to_string(),
                                    bb.square_list()));
            }
        }
    }
    None
}

fn test_piece_counts(board: &Board) -> Option<String> {
    for &side in &[WHITE, BLACK] {
        // non-promoting pieces
        for &(kind, min, max) in [(PAWN, 0, 8), (KING, 1, 1)].iter() {
            let piece = kind.pc(side);
            let count = board.bb_pc(piece).pop_count();
            if count < min {
                return Some(format!("Error: too few {}: found {} (min: {})",
                                    piece.string_plural(),
                                    count,
                                    min));
            } else if count > max {
                return Some(format!("Error: too many {}: found {} (max: {})",
                                    piece.string_plural(),
                                    count,
                                    max));
            }
        }

        // promoting pieces (max determined by remaining pawns)
        let pawn_count = board.bb_pc(PAWN.pc(side)).pop_count();

        for &(kind, max) in [(KNIGHT, 10), (BISHOP, 10), (ROOK, 10), (QUEEN, 9)].iter() {
            let piece = kind.pc(side);
            let count = board.bb_pc(piece).pop_count();
            let actual_max = max - pawn_count;
            if count > actual_max {
                return Some(format!("Error: too many {}: found {} (max: {})",
                                    piece.string_plural(),
                                    count,
                                    actual_max));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use board::*;

    #[test]
    fn checks_castling_rights_king_position_1() {
        let board = Board::from_fen("rnbq1bnr/ppppkppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: black cannot castle as king has moved");
    }

    #[test]
    fn checks_castling_rights_king_position_2() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w qK").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: white cannot castle as king has moved");
    }

    #[test]
    fn checks_castling_rights_rook_position_1() {
        let board = Board::from_fen("1nbqkbnr/rppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: black cannot castle queen-side as rook has moved");
    }

    #[test]
    fn checks_castling_rights_rook_position_2() {
        let board = Board::from_fen("rnbqkbr1/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w k").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: black cannot castle king-side as rook has moved");
    }

    #[test]
    fn checks_castling_rights_rook_position_3() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/1RBQKBNR w Q").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: white cannot castle queen-side as rook has moved");
    }

    #[test]
    fn checks_castling_rights_rook_position_4() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1N w K").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: white cannot castle king-side as rook has moved");
    }

    #[test]
    fn checks_ep_square_wrong_attacking_side_1() {
        // wrong attacking side: black
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR w - c3")
            .unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: en-passant square is c3 but side to move is white");
    }

    #[test]
    fn checks_ep_square_wrong_attacking_side_2() {

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b - c3")
            .unwrap();
        assert!(test(&board).is_none());
    }

    #[test]
    fn checks_ep_square_wrong_attacking_side_3() {

        // wrong attacking side: white
        let board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/8/8/PPPPPPPP/RNBQKBNR b - c6")
            .unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: en-passant square is c6 but side to move is black");
    }

    #[test]
    fn checks_ep_square_wrong_attacking_side_4() {

        let board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/8/8/PPPPPPPP/RNBQKBNR w - c6")
            .unwrap();
        assert!(test(&board).is_none());
    }

    #[test]
    fn checks_ep_square_no_target_1() {
        // No target white pawn
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - c3").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: en-passant square is c3 but no white pawn at c4");
    }

    #[test]
    fn checks_ep_square_no_target_2() {
        // No target black pawn
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - c6").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: en-passant square is c6 but no black pawn at c5");
    }

    #[test]
    fn checks_pawn_invalid_rows_1() {
        let board = Board::from_fen("rnbqkbnr/8/p7/8/8/8/8/RNBQKBNP w -").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: white pawn in invalid position: [h1]");
    }

    #[test]
    fn checks_pawn_invalid_rows_2() {

        let board = Board::from_fen("rnPqkbnr/8/8/8/8/8/8/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: white pawn in invalid position: [c8]");
    }

    #[test]
    fn checks_ok() {
        let board = Board::from_fen(STARTING_POSITION_FEN).unwrap();
        assert!(test(&board).is_none());
    }

    #[test]
    fn checks_pawn_count() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/p7/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many black pawns: found 9 (max: 8)");
    }

    #[test]
    fn checks_king_count_1() {
        let board = Board::from_fen("rnbqkknr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many black kings: found 2 (max: 1)");
    }

    #[test]
    fn checks_king_count_2() {

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQQBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too few white kings: found 0 (min: 1)");
    }

    #[test]
    fn checks_queen_count_1() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/7Q/QQQQQQQQ/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many white queens: found 10 (max: 9)");
    }

    #[test]
    fn checks_queen_count_2() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/7Q/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many white queens: found 2 (max: 1)");
    }

    #[test]
    fn checks_knights_count_2() {
        let board = Board::from_fen("rnbqkbnr/ppppnnnn/n7/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many black knights: found 7 (max: 6)");
    }

    #[test]
    fn checks_bishops_count_2() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/7B/PBBBBBBB/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many white bishops: found 10 (max: 9)");
    }

    #[test]
    fn checks_rooks_count_2() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/7R/PPPRRRRR/RNBQKBNR").unwrap();
        assert_eq!(test(&board).unwrap(),
                   "Error: too many white rooks: found 8 (max: 7)");
    }
}
