use mv_list::MoveList;
use piece::*;
use bb::*;
use board::Board;
use square::Square;
use gen::statics::{KING_MOVES, KNIGHT_MOVES};

pub fn king_moves<L: MoveList>(board: &Board, to_mask: BB, list: &mut L) {
    let stm = board.state().stm;
    let piece = KING.pc(stm);
    let movers = board.bb_pc(piece);
    let enemy = board.bb_side(stm.flip());
    let friendly = board.bb_side(stm);

    debug_assert_eq!(movers.pop_count(), 1);
    let from = movers.bitscan();

    let targets = king_moves_from_sq(from) & to_mask & !friendly;
    list.add_moves(from, targets, enemy);
}

pub fn knight_moves<L: MoveList>(board: &Board, to_mask: BB, from_mask: BB, list: &mut L) {
    let stm = board.state().stm;
    let piece = KNIGHT.pc(stm);
    let movers = board.bb_pc(piece) & from_mask;
    let enemy = board.bb_side(stm.flip());
    let friendly = board.bb_side(stm);

    for (from, _) in movers.iter() {
        let targets = knight_moves_from_sq(from) & to_mask & !friendly;
        list.add_moves(from, targets, enemy);
    }
}

#[inline]
pub fn king_moves_from_sq(sq: Square) -> BB {
    unsafe {
        return *KING_MOVES.get_unchecked(sq.to_usize());
    }
}

#[inline]
pub fn knight_moves_from_sq(sq: Square) -> BB {
    unsafe {
        return *KNIGHT_MOVES.get_unchecked(sq.to_usize());
    }
}

#[cfg(test)]
mod test {
    use gen::util::assert_list_includes_moves;
    use super::*;
    use bb::EMPTY;
    use board::STARTING_POSITION_FEN;
    use mv_list::MoveVec;
    use square::*;
    use gen::statics::init_all;

    #[test]
    fn king_pushes() {
        init_all();

        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !EMPTY, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e1d1"]);
    }

    #[test]
    fn king_captures() {
        init_all();

        let board = &Board::from_fen("rnbqkbnr/pppppPpp/8/8/8/8/PPPPPPPP/RNB1KBNR b").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !EMPTY, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e8xf7"]);
    }

    #[test]
    fn king_moves_with_mask() {
        init_all();

        let board = &Board::from_fen("1nbqkbn1/ppprpppp/8/8/8/3p4/5p2/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !BB::new(E2), &mut list);
        assert_list_includes_moves(&list, &["e1d2", "e1xf2"]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn knight_pushes() {
        init_all();

        let board = &Board::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();
        knight_moves::<MoveVec>(board, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b1a3", "b1c3", "g1f3", "g1h3"]);
    }

    #[test]
    fn knight_captures() {
        init_all();

        let board = &Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();
        knight_moves::<MoveVec>(board, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b8xa6"]);
    }
}
