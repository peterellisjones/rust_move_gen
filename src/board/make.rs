use mv::Move;
use castle::*;
use bb::*;
use super::{Board, State};
use piece::*;
use square::*;
use castling_rights::*;
use side::Side;

// If move intersects this mask, then remove castling right
const CASTLE_MASKS: [BB; 4] = [BB(1u64 | (1u64 << 4)), // WHITE QS: A1 + E1
                               BB((1u64 | (1u64 << 4)) << 56), // BLACK QS: A8 + E8
                               BB((1u64 << 4) | (1u64 << 7)), // WHITE KS: E1 + H1
                               BB(((1u64 << 4) | (1u64 << 7)) << 56) /* BLACK KS: E8 + H8 */];

impl Board {
    /// Returns piece captured and square if any
    pub fn make(&mut self, mv: Move) -> Option<(Piece, Square)> {
        let stm = self.state.stm;
        let initial_state = self.state.clone();

        self.state.half_move_clock += 1;
        // increment full move clock if black moved
        self.state.full_move_number += self.state.stm.to_usize();
        self.state.stm = self.state.stm.flip();

        self.state.ep_square = None;
        let mut captured = None;

        let mut xor_key = 0u64;

        if mv.is_castle() {
            let castle = mv.castle();
            self.state.castling_rights.clear_side(stm);
            self.make_castle(castle, stm);

            xor_key ^= self.hash.castle(castle, stm);
        } else {
            let from = mv.from();
            let to = mv.to();

            if mv.is_capture() {
                let capture_sq = if mv.is_ep_capture() {
                    from.along_row_with_col(to)
                } else {
                    to
                };

                debug_assert!(self.at(capture_sq).is_some());

                let captured_piece = self.at(capture_sq).unwrap();
                self.remove_piece(capture_sq);

                captured = Some((captured_piece, capture_sq));

                xor_key ^= self.hash.capture(captured_piece, capture_sq);
            }

            let mover = self.at(from).unwrap();
            let move_mask = self.move_piece(from, to);
            let mut updated_mover = mover;

            // if double pawn push (pawn move that travels two rows), set ep square
            if mover.kind() == PAWN && mv.distance() == 16 {
                self.state.ep_square = Some(Square((to.raw() + from.raw()) >> 1));
            }

            if mv.is_promotion() {
                updated_mover = mv.promote_to().pc(stm);
                self.change_piece(to, updated_mover);
            }

            xor_key ^= self.hash.push(mover, from, updated_mover, to);

            for (i, mask) in CASTLE_MASKS.iter().enumerate() {
                if (move_mask & *mask) != EMPTY {
                    self.state.castling_rights.clear(CastlingRights(1 << i));
                }
            }
        }


        xor_key ^= self.hash.state(&initial_state, &self.state);

        self.key ^= xor_key;

        captured
    }

    pub fn unmake(&mut self,
                  mv: Move,
                  capture: Option<(Piece, Square)>,
                  original_state: &State,
                  original_hash_key: u64) {
        self.state = original_state.clone();
        self.key = original_hash_key;

        if mv.is_castle() {
            self.unmake_castle(mv.castle(), original_state.stm);
            return;
        }

        if mv.is_promotion() {
            let mover = PAWN.pc(original_state.stm);
            self.change_piece(mv.to(), mover);
        }

        self.move_piece(mv.to(), mv.from());

        if capture.is_some() {
            let (captured_piece, capture_sq) = capture.unwrap();
            self.put_piece(captured_piece, capture_sq);
        }
    }

    fn unmake_castle(&mut self, castle: Castle, stm: Side) {
        let (to, from) = castle_king_squares(stm, castle);
        self.move_piece(from, to);
        let (to, from) = castle_rook_squares(stm, castle);
        self.move_piece(from, to);
    }

    fn make_castle(&mut self, castle: Castle, stm: Side) {
        let (from, to) = castle_king_squares(stm, castle);
        self.move_piece(from, to);
        let (from, to) = castle_rook_squares(stm, castle);
        self.move_piece(from, to);
    }
}

#[cfg(test)]
mod test {
    use board::Board;
    use mv::Move;
    use integrity;
    use square::*;
    use piece::*;
    use castle::*;

    fn test_make_unmake(initial_fen: &'static str, expected_fen: &'static str, mv: Move) {
        let mut board = Board::from_fen(initial_fen).unwrap();
        assert!(integrity::test(&board).is_none());

        let state = board.state().clone();

        let initial_key = board.hash_key();

        let capture = board.make(mv);
        assert_eq!(board.to_string(),
                   Board::from_fen(expected_fen).unwrap().to_string());

        assert!(integrity::test(&board).is_none());

        board.unmake(mv, capture, &state, initial_key);
        assert_eq!(board.to_string(),
                   Board::from_fen(initial_fen).unwrap().to_string());
        assert!(integrity::test(&board).is_none());
    }

    #[test]
    fn test_hash() {
        let mut board_1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w QqKk - 1 1")
            .unwrap();
        let mut board_2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w QqKk - 1 1")
            .unwrap();
        board_1.make(Move::new_push(D2, D4));
        board_1.make(Move::new_push(B8, C6));
        board_1.make(Move::new_castle(QUEEN_SIDE));
        board_1.make(Move::new_capture(C6, D4));

        board_2.make(Move::new_castle(QUEEN_SIDE));
        board_2.make(Move::new_push(B8, C6));
        board_2.make(Move::new_push(D2, D4));
        board_2.make(Move::new_capture(C6, D4));

        assert_eq!(board_1.to_fen(), board_2.to_fen());
        assert_eq!(board_1.hash_key(), board_2.hash_key());
    }

    #[test]
    fn test_make_unmake_simple_push() {
        test_make_unmake("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
                         "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b QqKk - 1 1",
                         Move::new_push(B1, C3));
    }

    #[test]
    fn test_make_unmake_double_pawn_push() {
        test_make_unmake("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
                         "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b QqKk d3 1 1",
                         Move::new_push(D2, D4));
    }


    #[test]
    fn test_make_unmake_push_with_castle_invalidation() {
        test_make_unmake("rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
                         "1nbqkbnr/rppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b QKk - 1 1",
                         Move::new_push(A8, A7));
    }

    #[test]
    fn test_make_unmake_promotion() {
        test_make_unmake("rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPPp/RNBQKBN1 b Qqk",
                         "rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPP1/RNBQKBNq w Qqk - 1 2",
                         Move::new_promotion(H2, H1, QUEEN));
    }

    #[test]
    fn test_make_unmake_capture_promotion() {
        test_make_unmake("rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w QKqk",
                         "Nnbqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b QKk - 1 1",
                         Move::new_capture_promotion(B7, A8, KNIGHT));
    }

    #[test]
    fn test_make_unmake_ep_capture() {
        test_make_unmake("rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPP1PPPP/RNBQKBNR w QqKk e6",
                         "rnbqkbnr/pppp1ppp/4P3/8/8/8/PPP1PPPP/RNBQKBNR b QqKk - 1 1",
                         Move::new_ep_capture(D5, E6));
    }


    #[test]
    fn test_make_unmake_castle() {
        test_make_unmake("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR",
                         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR b qk - 1 1",
                         Move::new_castle(QUEEN_SIDE));
    }

    #[test]
    fn test_make_unmake_double_push() {
        test_make_unmake("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b",
                         "rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR w QqKk d6 1 2",
                         Move::new_push(D7, D5));
    }

    #[test]
    fn test_make_unmake_capture() {
        test_make_unmake("rnbqkbnr/pppppppp/7P/8/8/8/PPPPPPP1/RNBQKBNR",
                         "rnbqkbnr/ppppppPp/8/8/8/8/PPPPPPP1/RNBQKBNR b QqKk - 1 1",
                         Move::new_capture(H6, G7));
    }
}
