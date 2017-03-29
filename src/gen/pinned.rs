use side::Side;
use board::Board;
use bb::*;
use piece::*;
use super::slider::*;
use super::pawn::*;
use mv_list::MoveList;

// Generates moves along pin rays, and also returns bb of pinned pieces
pub fn pin_ray_moves<L: MoveList>(board: &Board,
                                  in_check: bool,
                                  capture_mask: BB,
                                  push_mask: BB,
                                  stm: Side,
                                  list: &mut L)
                                  -> BB {
    let move_mask = capture_mask | push_mask;
    let king = board.bb_pc(KING.pc(stm));
    let (enemy_diag, enemy_non_diag) = board.bb_sliders(stm.flip());

    let empty = board.bb_empty();
    let friendly = board.bb_side(stm);

    let pinned = pinned_pieces(king, empty, enemy_diag, enemy_non_diag) & friendly;

    // if in check pinned pieces cannot move
    if pinned == EMPTY || in_check {
        return pinned;
    }

    let (north_west_south_east, north_east_south_west) =
        diag_pin_rays_including_attackers(king, empty | pinned, enemy_diag);

    // sliders can never pass 'through' the king so we can calculate
    // moves for sliders on opposite sides of the king together
    if north_west_south_east != EMPTY {
        diag_slider_moves(board,
                          north_west_south_east & move_mask,
                          north_west_south_east,
                          list);
    }
    if north_east_south_west != EMPTY {
        diag_slider_moves(board,
                          north_east_south_west & move_mask,
                          north_east_south_west,
                          list);
    }
    let diag_rays = north_west_south_east | north_east_south_west;

    // impossible for pawns to "cross" rays so can do all at once
    if diag_rays & capture_mask != EMPTY {
        pawn_captures(board,
                      diag_rays & capture_mask,
                      diag_rays & push_mask,
                      diag_rays,
                      list);
    }

    let (north_south, east_west) =
        non_diag_pin_rays_including_attackers(king, empty | pinned, enemy_non_diag);


    if north_south != EMPTY {
        non_diag_slider_moves(board, north_south & move_mask, north_south, list);
    }
    if east_west != EMPTY {
        non_diag_slider_moves(board, east_west & move_mask, east_west, list);
    }
    let non_diag_rays = north_south | east_west;

    if non_diag_rays & push_mask != EMPTY {
        // impossible for pawns to "cross" rays so can do all at once
        // do not need to evaluate captures here since all captures are on diagonals
        pawn_pushes(board, non_diag_rays & push_mask, non_diag_rays, list);
    }

    pinned
}

#[cfg(test)]
mod test {
    use gen::statics::init_all;
    use super::*;
    use board::Board;
    use side::*;
    use mv_list::MoveVec;
    use unindent;

    #[test]
    fn test_pin_ray_moves() {
        init_all();

        let board = Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w QK - 3 9")
            .unwrap();

        let mut list = MoveVec::new();
        let pinned_pieces = pin_ray_moves(&board, false, !EMPTY, !EMPTY, WHITE, &mut list);
        let expected = unindent::unindent("
          ABCDEFGH
        8|........|8
        7|........|7
        6|........|6
        5|........|5
        4|........|4
        3|........|3
        2|...#....|2
        1|........|1
          ABCDEFGH
        ");

        assert_eq!(pinned_pieces.to_string(), expected);
    }

    #[test]
    fn test_pin_ray_moves_2() {
        init_all();

        let board = Board::from_fen("5k2/8/8/q7/8/2Q5/8/4K3 w - -").unwrap();

        let mut list = MoveVec::new();
        let pinned_pieces = pin_ray_moves(&board, false, !EMPTY, !EMPTY, WHITE, &mut list);
        let expected = unindent::unindent("
          ABCDEFGH
        8|........|8
        7|........|7
        6|........|6
        5|........|5
        4|........|4
        3|..#.....|3
        2|........|2
        1|........|1
          ABCDEFGH
        ");

        assert_eq!(pinned_pieces.to_string(), expected);
        assert_eq!(list.len(), 3);
    }
}
