use bb::*;
use castle::Castle;
use castling_rights::CastlingRights;
use mv_list::MoveList;
use position::Position;

pub fn castles<L: MoveList>(position: &Position, attacks: BB, list: &mut L) {
    const CASTLE_BLOCKING_SQUARES: [[BB; 2]; 2] = [
        [
            BB((1u64 << 1) + (1u64 << 2) + (1u64 << 3)), /* WHITE QS = B1 + C1 + D1 */
            BB((1u64 << 57) + (1u64 << 58) + (1u64 << 59)),
        ], /* BLACK QS = B8 + C8 + D1 */
        [
            BB((1u64 << 5) + (1u64 << 6)), // WHITE KS = F1 + G1
            BB((1u64 << 61) + (1u64 << 62)),
        ],
    ]; // BLACK KS = F8 + G8

    // squares that must be not attacked for a castle to take place
    const KING_SAFE_SQUARES: [[BB; 2]; 2] = [
        [
            BB((1u64 << 2) + (1u64 << 3) + (1u64 << 4)), /* WHITE QS = C1 + D1 + E1 */
            BB((1u64 << 58) + (1u64 << 59) + (1u64 << 60)),
        ], /* BLACK QS = C8 + D8  + E8 */
        [
            BB((1u64 << 4) + (1u64 << 5) + (1u64 << 6)), /* WHITE KS = E1 + F1 + G1 */
            BB((1u64 << 60) + (1u64 << 61) + (1u64 << 62)),
        ],
    ]; // BLACK KS = E8 + F8 + G8

    let stm = position.state().stm;
    let rights = position.state().castling_rights;
    let occupied_squares = position.bb_occupied();

    for castle in Castle::iter().filter(|c| rights.has(CastlingRights::from(*c, stm))) {
        // NOTE: should not need to check king and rook pos since
        // should not be able to castle once these are moved

        let blockers = CASTLE_BLOCKING_SQUARES[castle.to_usize()][stm.to_usize()];
        let king_safe = KING_SAFE_SQUARES[castle.to_usize()][stm.to_usize()];

        if (occupied_squares & blockers).any() | (attacks & king_safe).any() {
            continue;
        }

        list.add_castle(castle);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gen::attacked_squares_ignoring_ep;
    use gen::util::assert_list_includes_moves;
    use mv_list::MoveVec;

    #[test]
    fn no_castles() {
        let position = &Position::from_fen("rnbqkbnr/8/8/8/8/8/8/R3K2R w -").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);

        let position = &Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b QK").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_castle() {
        let position = &Position::from_fen("rnbqkbnr/8/8/8/8/8/8/R3K2R w K").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["O-O"]);

        let position = &Position::from_fen("r3kbnr/8/8/8/8/8/8/R3K2R b KQq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 1);
        assert_list_includes_moves(&list, &["O-O-O"]);
    }

    #[test]
    fn cant_castle_when_blocked() {
        let position = &Position::from_fen("rnbqkbnr/8/8/8/8/8/8/Rn2K2R w Qkq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);

        let position = &Position::from_fen("rnbqkb1r/8/8/8/8/8/8/Rn2K2R b Qkkq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn cant_castle_when_king_passes_through_attack_1() {
        let position = &Position::from_fen("rnbqkbnr/pppppppp/3r4/8/8/8/8/R3K2R w Qkq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn cant_castle_when_king_passes_through_attack_2() {
        let position = &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/1n6/R3K2R w Qkq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_castle_when_rook_passes_through_attack() {
        let position = &Position::from_fen("rnbqkbnr/pppppppp/1r6/8/8/8/8/R3K2R w Qkq").unwrap();
        let mut list = MoveVec::new();
        let attacks = attacked_squares_ignoring_ep(position.state().stm.flip(), position);
        castles::<MoveVec>(position, attacks, &mut list);
        assert_eq!(list.len(), 1);
    }
}
