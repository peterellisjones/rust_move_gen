use bb::*;
use mv_list::MoveAdder;
use piece::*;
use position::Position;

pub fn king_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    list: &mut L,
) {
    let stm = position.state().stm;
    let piece = KING.pc(stm);
    let movers = position.bb_pc(piece);

    debug_assert_eq!(movers.pop_count(), 1);
    let from = movers.bitscan();
    debug_assert_eq!(position.at(from), piece);

    let capture_targets = from.king_moves() & capture_mask;
    let push_targets = from.king_moves() & push_mask;

    list.add_captures(from, capture_targets);
    list.add_non_captures(from, push_targets);
}

pub fn king_captures<L: MoveAdder>(position: &Position, capture_mask: BB, list: &mut L) {
    let stm = position.state().stm;
    let piece = KING.pc(stm);
    let movers = position.bb_pc(piece);

    debug_assert_eq!(movers.pop_count(), 1);
    let from = movers.bitscan();
    debug_assert_eq!(position.at(from), piece);

    let capture_targets = from.king_moves() & capture_mask;

    list.add_captures(from, capture_targets);
}

pub fn knight_moves<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    from_mask: BB,
    list: &mut L,
) {
    let stm = position.state().stm;
    let piece = KNIGHT.pc(stm);
    let movers = position.bb_pc(piece) & from_mask;

    for (from, _) in movers.iter() {
        debug_assert_eq!(position.at(from), piece);
        let capture_targets = from.knight_moves() & capture_mask;
        let push_targets = from.knight_moves() & push_mask;

        list.add_captures(from, capture_targets);
        list.add_non_captures(from, push_targets);
    }
}

pub fn knight_captures<L: MoveAdder>(
    position: &Position,
    capture_mask: BB,
    from_mask: BB,
    list: &mut L,
) {
    let stm = position.state().stm;
    let piece = KNIGHT.pc(stm);
    let movers = position.bb_pc(piece) & from_mask;

    for (from, _) in movers.iter() {
        debug_assert_eq!(position.at(from), piece);
        let capture_targets = from.knight_moves() & capture_mask;

        list.add_captures(from, capture_targets);
    }
}

pub fn knight_moves_from_bb(knights: BB) -> BB {
    let attacks_right_one = (knights << 1) & !FILE_A;
    let attacks_right_two = (knights << 2) & !(FILE_A | FILE_B);
    let attacks_left_one = (knights >> 1) & !FILE_H;
    let attacks_left_two = (knights >> 2) & !(FILE_H | FILE_G);

    let attacks_one = attacks_right_one | attacks_left_one;
    let attacks_two = attacks_right_two | attacks_left_two;

    (attacks_one << 16) | (attacks_one >> 16) | (attacks_two << 8) | (attacks_two >> 8)
}

#[cfg(test)]
mod test {
    use super::*;
    use bb::EMPTY;
    use gen::util::assert_list_includes_moves;
    use mv_list::MoveVec;
    use position::STARTING_POSITION_FEN;
    use square::*;

    #[test]
    fn king_pushes() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w").unwrap();
        let mut list = MoveVec::new();
        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        king_moves::<MoveVec>(position, capture_mask, push_mask, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e1d1"]);
    }

    #[test]
    fn king_captures() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppPpp/8/8/8/8/PPPPPPPP/RNB1KBNR b").unwrap();
        let mut list = MoveVec::new();
        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        king_moves::<MoveVec>(position, capture_mask, push_mask, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e8xf7"]);
    }

    #[test]
    fn king_moves_with_mask() {
        let position = &Position::from_fen("1nbqkbn1/ppprpppp/8/8/8/3p4/5p2/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();
        let capture_mask = !BB::new(E2) & position.bb_side(position.state().stm.flip());
        let push_mask = !BB::new(E2) & position.bb_empty();

        king_moves::<MoveVec>(position, capture_mask, push_mask, &mut list);
        assert_list_includes_moves(&list, &["e1d2", "e1xf2"]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn knight_pushes() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();

        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        knight_moves::<MoveVec>(position, capture_mask, push_mask, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b1a3", "b1c3", "g1f3", "g1h3"]);
    }

    #[test]
    fn test_knight_moves_from_bb() {
        for (i, &moves) in KNIGHT_MOVES.iter().enumerate() {
            let bb = BB::new(Square::new(i));
            assert_eq!(knight_moves_from_bb(bb), moves);
        }
    }

    #[test]
    fn knight_captures() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();

        let capture_mask = position.bb_side(position.state().stm.flip());
        let push_mask = position.bb_empty();

        knight_moves::<MoveVec>(position, capture_mask, push_mask, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b8xa6"]);
    }
}
