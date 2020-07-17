use super::pawn::*;
use super::slider::*;
use bb::*;
use mv_list::MoveList;
use piece::*;
use position::Position;
use side::Side;

// Generates moves along pin rays, and also returns bb of pinned pieces
pub fn pin_ray_moves<L: MoveList>(
    position: &Position,
    capture_mask: BB,
    push_mask: BB,
    stm: Side,
    list: &mut L,
) -> BB {
    let move_mask = capture_mask | push_mask;
    let king = position.bb_pc(KING.pc(stm));
    let (enemy_diag, enemy_non_diag) = position.bb_sliders(stm.flip());

    let empty = position.bb_empty();
    let friendly = position.bb_side(stm);

    let pinned = pinned_pieces(king, empty, enemy_diag, enemy_non_diag) & friendly;

    let (north_west_south_east, north_east_south_west) =
        diag_pin_rays_including_attackers(king, empty | pinned, enemy_diag);

    let diag_rays = north_west_south_east | north_east_south_west;

    // impossible for pawns to "cross" rays so can do all at once
    if diag_rays & capture_mask != EMPTY {
        pawn_captures(
            position,
            diag_rays & capture_mask,
            diag_rays & push_mask,
            !diag_rays,
            list,
        );
    }

    let (north_south, east_west) =
        non_diag_pin_rays_including_attackers(king, empty | pinned, enemy_non_diag);

    if north_south != EMPTY {
        non_diag_slider_moves(position, north_south & move_mask, north_south, list);
    }
    if east_west != EMPTY {
        non_diag_slider_moves(position, east_west & move_mask, east_west, list);
    }
    let non_diag_rays = north_south | east_west;

    if non_diag_rays & push_mask != EMPTY {
        // impossible for pawns to "cross" rays so can do all at once
        // do not need to evaluate captures here since all captures are on diagonals
        pawn_pushes(position, non_diag_rays & push_mask, !non_diag_rays, list);
    }

    pinned
}

#[cfg(test)]
mod test {
    use mv_list::MoveVec;
    use position::Position;
    use side::*;
    use unindent;

    #[test]
    fn test_pin_ray_moves() {
        let position =
            Position::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w QK - 3 9")
                .unwrap();

        let mut list = MoveVec::new();
        let pinned_pieces = pin_ray_moves(&position, false, !EMPTY, !EMPTY, WHITE, &mut list);
        let expected = unindent::unindent(
            "
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
        ",
        );

        assert_eq!(pinned_pieces.to_string(), expected);
    }

    #[test]
    fn test_pin_ray_moves_2() {
        let position = Position::from_fen("5k2/8/8/q7/8/2Q5/8/4K3 w - -").unwrap();

        let mut list = MoveVec::new();
        let pinned_pieces = pin_ray_moves(&position, false, !EMPTY, !EMPTY, WHITE, &mut list);
        let expected = unindent::unindent(
            "
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
        ",
        );

        assert_eq!(pinned_pieces.to_string(), expected);
        assert_eq!(list.len(), 3);
    }
}
