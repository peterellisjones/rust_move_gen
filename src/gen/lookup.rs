use mv_list::MoveList;
use piece::*;
use bb::*;
use board::Board;
use square::Square;

#[inline]
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

#[inline]
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

#[inline]
pub fn knight_moves_from_bb(knights: BB) -> BB {
    let attacks_up_two = knights.rot_left(16) & !(ROW_1 | ROW_2);
    let attacks_up_one = knights.rot_left(8) & !ROW_1;
    let attacks_down_one = knights.rot_left(56) & !ROW_8;
    let attacks_down_two = knights.rot_left(48) & !(ROW_8 | ROW_7);

    let attacks_left_one = (attacks_up_two | attacks_down_two).rot_left(63) & !FILE_H;
    let attacks_left_two = (attacks_up_one | attacks_down_one).rot_left(62) & !(FILE_H | FILE_G);

    let attacks_right_one = (attacks_up_two | attacks_down_two).rot_left(1) & !FILE_A;
    let attacks_right_two = (attacks_up_one | attacks_down_one).rot_left(2) & !(FILE_A | FILE_B);

    attacks_left_one | attacks_right_one | attacks_left_two | attacks_right_two
}

#[cfg(test)]
mod test {
    use gen::util::assert_list_includes_moves;
    use super::*;
    use bb::EMPTY;
    use board::STARTING_POSITION_FEN;
    use mv_list::MoveVec;
    use square::*;

    #[test]
    fn king_pushes() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !EMPTY, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e1d1"]);
    }

    #[test]
    fn king_captures() {
        let board = &Board::from_fen("rnbqkbnr/pppppPpp/8/8/8/8/PPPPPPPP/RNB1KBNR b").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !EMPTY, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["e8xf7"]);
    }

    #[test]
    fn king_moves_with_mask() {
        let board = &Board::from_fen("1nbqkbn1/ppprpppp/8/8/8/3p4/5p2/RNBQKBNR w").unwrap();
        let mut list = MoveVec::new();
        king_moves::<MoveVec>(board, !BB::new(E2), &mut list);
        assert_list_includes_moves(&list, &["e1d2", "e1xf2"]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn knight_pushes() {
        let board = &Board::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();
        knight_moves::<MoveVec>(board, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b1a3", "b1c3", "g1f3", "g1h3"]);
    }

    #[test]
    fn test_knight_moves_from_bb() {
        for i in 0..64 {
            let bb = BB::new(Square::new(i));
            let actual_moves = KNIGHT_MOVES[i];
            assert_eq!(knight_moves_from_bb(bb), actual_moves);
        }
    }

    #[test]
    fn knight_captures() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR b").unwrap();
        let mut list = MoveVec::new();
        knight_moves::<MoveVec>(board, !EMPTY, !EMPTY, &mut list);
        assert_eq!(list.len(), 4);
        assert_list_includes_moves(&list, &["b8xa6"]);
    }
}


pub const KING_MOVES: [BB; 64] = [BB(0x0000000000000302u64),
                                  BB(0x0000000000000705u64),
                                  BB(0x0000000000000E0Au64),
                                  BB(0x0000000000001C14u64),
                                  BB(0x0000000000003828u64),
                                  BB(0x0000000000007050u64),
                                  BB(0x000000000000E0A0u64),
                                  BB(0x000000000000C040u64),
                                  BB(0x0000000000030203u64),
                                  BB(0x0000000000070507u64),
                                  BB(0x00000000000E0A0Eu64),
                                  BB(0x00000000001C141Cu64),
                                  BB(0x0000000000382838u64),
                                  BB(0x0000000000705070u64),
                                  BB(0x0000000000E0A0E0u64),
                                  BB(0x0000000000C040C0u64),
                                  BB(0x0000000003020300u64),
                                  BB(0x0000000007050700u64),
                                  BB(0x000000000E0A0E00u64),
                                  BB(0x000000001C141C00u64),
                                  BB(0x0000000038283800u64),
                                  BB(0x0000000070507000u64),
                                  BB(0x00000000E0A0E000u64),
                                  BB(0x00000000C040C000u64),
                                  BB(0x0000000302030000u64),
                                  BB(0x0000000705070000u64),
                                  BB(0x0000000E0A0E0000u64),
                                  BB(0x0000001C141C0000u64),
                                  BB(0x0000003828380000u64),
                                  BB(0x0000007050700000u64),
                                  BB(0x000000E0A0E00000u64),
                                  BB(0x000000C040C00000u64),
                                  BB(0x0000030203000000u64),
                                  BB(0x0000070507000000u64),
                                  BB(0x00000E0A0E000000u64),
                                  BB(0x00001C141C000000u64),
                                  BB(0x0000382838000000u64),
                                  BB(0x0000705070000000u64),
                                  BB(0x0000E0A0E0000000u64),
                                  BB(0x0000C040C0000000u64),
                                  BB(0x0003020300000000u64),
                                  BB(0x0007050700000000u64),
                                  BB(0x000E0A0E00000000u64),
                                  BB(0x001C141C00000000u64),
                                  BB(0x0038283800000000u64),
                                  BB(0x0070507000000000u64),
                                  BB(0x00E0A0E000000000u64),
                                  BB(0x00C040C000000000u64),
                                  BB(0x0302030000000000u64),
                                  BB(0x0705070000000000u64),
                                  BB(0x0E0A0E0000000000u64),
                                  BB(0x1C141C0000000000u64),
                                  BB(0x3828380000000000u64),
                                  BB(0x7050700000000000u64),
                                  BB(0xE0A0E00000000000u64),
                                  BB(0xC040C00000000000u64),
                                  BB(0x0203000000000000u64),
                                  BB(0x0507000000000000u64),
                                  BB(0x0A0E000000000000u64),
                                  BB(0x141C000000000000u64),
                                  BB(0x2838000000000000u64),
                                  BB(0x5070000000000000u64),
                                  BB(0xA0E0000000000000u64),
                                  BB(0x40C0000000000000u64)];

pub const KNIGHT_MOVES: [BB; 64] = [BB(0x0000000000020400u64),
                                    BB(0x0000000000050800u64),
                                    BB(0x00000000000A1100u64),
                                    BB(0x0000000000142200u64),
                                    BB(0x0000000000284400u64),
                                    BB(0x0000000000508800u64),
                                    BB(0x0000000000A01000u64),
                                    BB(0x0000000000402000u64),
                                    BB(0x0000000002040004u64),
                                    BB(0x0000000005080008u64),
                                    BB(0x000000000A110011u64),
                                    BB(0x0000000014220022u64),
                                    BB(0x0000000028440044u64),
                                    BB(0x0000000050880088u64),
                                    BB(0x00000000A0100010u64),
                                    BB(0x0000000040200020u64),
                                    BB(0x0000000204000402u64),
                                    BB(0x0000000508000805u64),
                                    BB(0x0000000A1100110Au64),
                                    BB(0x0000001422002214u64),
                                    BB(0x0000002844004428u64),
                                    BB(0x0000005088008850u64),
                                    BB(0x000000A0100010A0u64),
                                    BB(0x0000004020002040u64),
                                    BB(0x0000020400040200u64),
                                    BB(0x0000050800080500u64),
                                    BB(0x00000A1100110A00u64),
                                    BB(0x0000142200221400u64),
                                    BB(0x0000284400442800u64),
                                    BB(0x0000508800885000u64),
                                    BB(0x0000A0100010A000u64),
                                    BB(0x0000402000204000u64),
                                    BB(0x0002040004020000u64),
                                    BB(0x0005080008050000u64),
                                    BB(0x000A1100110A0000u64),
                                    BB(0x0014220022140000u64),
                                    BB(0x0028440044280000u64),
                                    BB(0x0050880088500000u64),
                                    BB(0x00A0100010A00000u64),
                                    BB(0x0040200020400000u64),
                                    BB(0x0204000402000000u64),
                                    BB(0x0508000805000000u64),
                                    BB(0x0A1100110A000000u64),
                                    BB(0x1422002214000000u64),
                                    BB(0x2844004428000000u64),
                                    BB(0x5088008850000000u64),
                                    BB(0xA0100010A0000000u64),
                                    BB(0x4020002040000000u64),
                                    BB(0x0400040200000000u64),
                                    BB(0x0800080500000000u64),
                                    BB(0x1100110A00000000u64),
                                    BB(0x2200221400000000u64),
                                    BB(0x4400442800000000u64),
                                    BB(0x8800885000000000u64),
                                    BB(0x100010A000000000u64),
                                    BB(0x2000204000000000u64),
                                    BB(0x0004020000000000u64),
                                    BB(0x0008050000000000u64),
                                    BB(0x00110A0000000000u64),
                                    BB(0x0022140000000000u64),
                                    BB(0x0044280000000000u64),
                                    BB(0x0088500000000000u64),
                                    BB(0x0010A00000000000u64),
                                    BB(0x0020400000000000u64)];
