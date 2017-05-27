mod pawn;
mod lookup;
mod util;
pub mod slider;
mod castle;
mod attacks;
mod pinned;

use self::pawn::*;
use self::lookup::*;
use self::slider::*;
use self::attacks::*;
use self::castle::*;
use self::pinned::*;
use piece::*;

use board::Board;
use bb::EMPTY;
use mv_list::MoveList;
use piece::KING;

pub use self::attacks::king_danger_squares;

/// Adds legal moves to the provided MoveList. Returns true if mover is in check
pub fn legal_moves<L: MoveList>(board: &Board, list: &mut L) -> bool {
    let stm = board.state().stm;
    let kings = board.bb_pc(KING.pc(stm));

    // We always need legal king moves
    let attacked_squares = king_danger_squares(kings, stm.flip(), board);
    king_moves(board, !attacked_squares, list);

    let king_sq = kings.bitscan();

    // Don't need to calculate checkers if no attacks on king
    let (king_attacks_count, checkers) = if (attacked_squares & kings).any() {
        let checkers = checks_to_sq(king_sq, stm.flip(), board);
        (checkers.pop_count(), checkers)
    } else {
        (0, EMPTY)
    };

    // capture_mask and push_mask represent squares our pieces are allowed to move to or capture,
    // respectively. The difference between the two is only important for pawn EP captures
    let mut capture_mask = !EMPTY;
    let mut push_mask = !EMPTY;

    if king_attacks_count > 1 {
        // multiple attackers... only solutions are king moves
        return true;
    } else if king_attacks_count == 1 {
        // if ony one attacker, we can try attacking the attacker with
        // our other pieces.
        capture_mask = checkers;

        push_mask = EMPTY;

        // If the piece giving check is a slider, we can additionally attempt
        // to block the sliding piece;
        let checker_sq = checkers.bitscan();
        let checker = board.at(checker_sq).unwrap();

        if checker.is_slider() {
            // This branch is rare
            if checker.kind() != ROOK {
                push_mask |= slider_diag_rays_to_squares(kings, checkers, board);
            }
            if checker.kind() != BISHOP {
                push_mask |= slider_non_diag_rays_to_squares(kings, checkers, board);
            }
        }
    } else {
        // Not in check so can generate castles
        // impossible for castles to be affected by pins
        // so we don't need to consider pins here
        castles(board, attacked_squares, list);
    }

    let in_check = king_attacks_count > 0;

    // Generate rest of the moves, filtering movable squares
    let pinned = pin_ray_moves(board, in_check, capture_mask, push_mask, stm, list);

    let move_mask = capture_mask | push_mask;

    // generate moves for non-pinned pieces
    slider_moves(board, move_mask, !pinned, list);
    knight_moves(board, move_mask, !pinned, list);
    pawn_moves(board, capture_mask, push_mask, !pinned, list);

    in_check
}

#[cfg(test)]
mod test {
    use super::*;
    use mv_list::MoveVec;
    use board::STARTING_POSITION_FEN;

    macro_rules! test_gen {
        ($name:ident, $moves:expr, $fen:expr) => {
            #[test]
            fn $name() {
                let mut list = MoveVec::new();
                let board = &Board::from_fen($fen).unwrap();
                legal_moves::<MoveVec>(board, &mut list);
                if  list.len() != $moves {
                    println!("Moves missing: {}", list);
                    println!("{}", board);
                }
                assert_eq!(list.len(), $moves);
            }
        }
    }

    test_gen!(test_starting_position, 20, STARTING_POSITION_FEN);
    test_gen!(test_debug_1, 8, "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b QK - 3 2");
    test_gen!(test_debug_2, 8, "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 5 3");
    test_gen!(test_debug_3,
              19,
              "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w QqKk - 2 2");
    test_gen!(test_debug_4,
              5,
              "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b QqKk - 3 2");
    test_gen!(test_debug_5,
              44,
              "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b QK - 3 2");
    test_gen!(test_debug_6,
              39,
              "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w QK - 3 9");

    test_gen!(test_debug_7, 8, "5k2/8/8/q7/8/2Q5/8/4K3 w - -");

    test_gen!(test_debug_8, 9, "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4");
}
