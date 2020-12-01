use super::State;
use castling_rights::*;
use piece::*;
use side::*;
use square;
use square::Square;

pub fn from_fen(fen: &str) -> Result<([Piece; 64], State), String> {
    let mut state = State {
        castling_rights: ALL_RIGHTS,
        ep_square: None,
        stm: WHITE,
        half_move_clock: 0,
        full_move_number: 1,
    };

    let parts: Vec<&str> = fen.split(' ').collect();
    if parts.is_empty() {
        return Err(format!("Not enough fields for FEN: {}", fen));
    }

    let grid = parse_rows(parts[0])?;

    if parts.len() > 1 {
        state.stm = parse_stm(parts[1])?;
    }

    if parts.len() > 2 {
        state.castling_rights = parse_castling_rights(parts[2])?;
    }

    if parts.len() > 3 {
        state.ep_square = Square::parse(parts[3])?;
    }

    if parts.len() > 4 && parts[4] != "-" {
        match parts[4].parse::<usize>() {
            Ok(hmc) => state.half_move_clock = hmc,
            Err(err) => return Err(err.to_string()),
        }
    }

    if parts.len() > 5 && parts[5] != "-" {
        match parts[5].parse::<usize>() {
            Ok(fmn) => state.full_move_number = fmn,
            Err(err) => return Err(err.to_string()),
        }
    }

    Ok((grid, state))
}

pub fn to_fen(grid: &[Piece; 64], state: &State) -> String {
    let mut fen = String::new();

    for row in 0..8 {
        let mut empties = 0;
        for col in 0..8 {
            let piece = grid[Square::from(7 - row, col).to_usize()];
            if piece.is_none() {
                empties += 1;
            } else {
                if empties > 0 {
                    fen += &empties.to_string();
                    empties = 0;
                }
                fen.push(piece.to_char());
            }
        }
        if empties > 0 {
            fen += &empties.to_string();
        }
        if row != 7 {
            fen.push('/')
        }
    }

    fen.push(' ');
    fen.push(state.stm.to_char());
    fen.push(' ');
    fen += &state.castling_rights.to_string();
    fen.push(' ');
    fen += &(if let Some(sq) = state.ep_square {
        sq.to_string()
    } else {
        "-".to_string()
    });
    fen.push(' ');
    fen += &state.half_move_clock.to_string();
    fen.push(' ');
    fen += &state.full_move_number.to_string();

    fen
}

fn parse_stm(s: &str) -> Result<Side, String> {
    match s.chars().next() {
        Some(c) => Side::parse(c),
        None => Err("String too short".to_string()),
    }
}

fn parse_castling_rights(s: &str) -> Result<CastlingRights, String> {
    if s == "-" {
        return Ok(NO_RIGHTS);
    }
    let mut rights = NO_RIGHTS;

    for c in s.chars() {
        rights.set(CastlingRights::parse(c)?)
    }

    Ok(rights)
}

fn parse_rows(fen: &str) -> Result<[Piece; 64], String> {
    let mut grid = [NULL_PIECE; 64];

    for (i, row) in fen.split('/').enumerate() {
        if i >= 8 {
            break;
        }
        if let Some(err) = parse_row(row, 7 - i, &mut grid) {
            return Err(err);
        }
    }

    Ok(grid)
}

fn parse_row(row_str: &str, row: usize, grid: &mut [Piece; 64]) -> Option<String> {
    let mut col = 0;
    for c in row_str.chars() {
        if ('1'..='8').contains(&c) {
            col += c as usize - '1' as usize;
        } else {
            if col >= 8 {
                return Some(format!("Too many pieces on row {}", row + 1));
            }
            match Piece::parse(c) {
                Ok(pc) => {
                    let sq = Square::from(row as square::Internal, col as square::Internal);
                    grid[sq.to_usize()] = pc;
                }
                Err(err) => {
                    return Some(err);
                }
            }
        }
        col += 1;
    }

    None
}

#[cfg(test)]
mod test {
    use castle::*;
    use castling_rights::*;
    use position::*;
    use square::*;
    use unindent;

    #[test]
    fn parse_convert_to_fen_1() {
        let result = Position::from_fen(STARTING_POSITION_FEN);
        assert!(result.is_ok());

        let position = result.unwrap();

        let fen = position.to_fen();
        assert_eq!(fen, STARTING_POSITION_FEN);
    }

    #[test]
    fn parse_convert_to_fen_2() {
        let initial_fen = "rnbqkbnr/pppppp2/6pp/8/8/PP5P/2PPPPP1/RNBQKBNR w \
                           QqKk - 0 1";
        let result = Position::from_fen(initial_fen);
        assert!(result.is_ok());

        let position = result.unwrap();

        let fen = position.to_fen();
        assert_eq!(fen, initial_fen);
    }

    #[test]
    fn parse_convert_to_fen_3() {
        let initial_fen = "r3k2r/1b4bq/8/8/8/8/7B/R4RK1 b qk - 0 1";
        let result = Position::from_fen(initial_fen);
        assert!(result.is_ok());

        let position = result.unwrap();

        let fen = position.to_fen();
        assert_eq!(fen, initial_fen);
    }

    #[test]
    fn parse_parse_with_starting_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());

        let position = result.unwrap();

        let expected = unindent::unindent(
            "
          ABCDEFGH
        8|rnbqkbnr|8     side to move: white
        7|pppppppp|7  castling rights: QqKk
        6|........|6       en-passant: -
        5|........|5  half-move clock: 0
        4|........|4 full-move number: 1
        3|........|3              FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1
        2|PPPPPPPP|2
        1|RNBQKBNR|1
          ABCDEFGH
        ",
        );
        assert_eq!(position.to_string(), expected);
    }

    #[test]
    fn parse_with_default_state() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());

        let position = result.unwrap();

        assert_eq!(position.state().castling_rights, ALL_RIGHTS);
        assert_eq!(position.state().half_move_clock, 0);
        assert_eq!(position.state().full_move_number, 1);
    }

    #[test]
    fn parse_parse_with_random_fen() {
        let fen = "8/8/7p/3KNN1k/2p4p/8/3P2p1/8";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());

        let position = result.unwrap();

        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|........|8     side to move: white
            7|........|7  castling rights: QqKk
            6|.......p|6       en-passant: -
            5|...KNN.k|5  half-move clock: 0
            4|..p....p|4 full-move number: 1
            3|........|3              FEN: 8/8/7p/3KNN1k/2p4p/8/3P2p1/8 w QqKk - 0 1
            2|...P..p.|2
            1|........|1
              ABCDEFGH
        ",
        );

        assert_eq!(position.to_string(), expected);
    }

    #[test]
    fn parse_with_stm_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().stm, WHITE);
    }

    #[test]
    fn parse_with_stm_2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().stm, BLACK);
    }

    #[test]
    fn parse_with_ep_square_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert!(position.state().ep_square.is_none());
    }

    #[test]
    fn parse_with_ep_square_2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - c3";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().ep_square.unwrap(), C3);
    }

    #[test]
    fn parse_with_half_move_clock_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().half_move_clock, 0);
    }

    #[test]
    fn parse_with_half_move_clock_2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 23";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().half_move_clock, 23);
    }

    #[test]
    fn parse_with_full_move_number_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();

        assert_eq!(position.state().full_move_number, 1);
    }

    #[test]
    fn parse_with_full_move_number_2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 45";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().full_move_number, 45);
    }

    #[test]
    fn parse_with_castling_rights_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w -";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().castling_rights, NO_RIGHTS);
    }

    #[test]
    fn parse_with_castling_rights_2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b Qk";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        let mut expected_rights = WHITE_QS;
        expected_rights.add(KING_SIDE, BLACK);
        assert_eq!(position.state().castling_rights, expected_rights);
    }

    #[test]
    fn parse_with_castling_rights_3() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QkKq";
        let result = Position::from_fen(fen);
        assert!(result.is_ok());
        let position = result.unwrap();
        assert_eq!(position.state().castling_rights, ALL_RIGHTS);
    }

    #[test]
    fn parse_with_castling_rights_4() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b f";
        let result = Position::from_fen(fen);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid castle: f");
    }
}
