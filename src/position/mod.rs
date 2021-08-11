pub mod fen;
pub mod make;

use self::fen::*;
use super::util::grid_to_string_with_props;
use crate::bb::*;
use crate::castling_rights::*;
use crate::hash::{DEFAULT_ZOBRISH_HASH, Zobrist};
use crate::mv_list::PieceSquareTable;
use crate::piece::*;
use crate::side::Side;
use crate::side::*;
use crate::square::{Square, SquareInternal};
use std::fmt;

use std;

pub const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1";

/// State encodes all game state except position
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,
    pub stm: Side,
    pub full_move_number: u16,
    pub half_move_clock: u8,
}

impl Default for State {
    fn default() -> Self {
        State {
            castling_rights: NO_RIGHTS,
            ep_square: None,
            stm: WHITE,
            full_move_number: 1,
            half_move_clock: 0,
        }
    }
}

/// Position encodes all positional information and non-positional game state
pub struct Position {
    // grid is an array representation of position
    grid: [Piece; 64],
    // bb_sides represents a bitboard for each side
    bb_sides: [BB; 2],
    // bb_pieces represents a bitboard for each piece
    bb_pieces: [BB; 12],
    // state represents non-positional game state (eg side to move)
    state: State,

    key: u64,

    hash: &'static Zobrist,
}

impl std::clone::Clone for Position {
    fn clone(&self) -> Self {
        Position {
            grid: self.grid,
            bb_sides: self.bb_sides,
            bb_pieces: self.bb_pieces,
            state: self.state,
            key: self.key,
            hash: self.hash,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let props = vec![
            ("    side to move", self.state.stm.to_str().to_string()),
            (" castling rights", self.state.castling_rights.to_string()),
            (
                "      en-passant",
                self.state
                    .ep_square
                    .map_or("-".to_string(), |s| s.to_string()),
            ),
            (" half-move clock", self.state.half_move_clock.to_string()),
            ("full-move number", self.state.full_move_number.to_string()),
            ("             FEN", self.to_fen()),
            ("             KEY", format!("{:016X}", self.key)),
        ];
        let s = grid_to_string_with_props(
            |sq: Square| -> char {
                let pc = self.at(sq);
                if pc.is_none() { '.' } else { pc.to_char() }
            },
            props.as_slice(),
        );

        write!(f, "{}", &s)
    }
}

impl Position {
    pub fn new(grid: [Piece; 64], state: State) -> Position {
        let mut bb_pieces = [EMPTY; 12];
        let mut bb_sides = [EMPTY; 2];

        for (idx, pc) in grid.iter().enumerate().filter(|&(_, &pc)| pc.is_some()) {
            let bb_mask = BB::new(Square::new(idx as SquareInternal));
            bb_sides[pc.side().raw()] |= bb_mask;
            bb_pieces[pc.to_usize()] |= bb_mask;
        }

        let hash = &DEFAULT_ZOBRISH_HASH;
        let key = hash.position(&grid, &state);

        Position {
            grid,
            bb_pieces,
            bb_sides,
            state,
            hash: &DEFAULT_ZOBRISH_HASH,
            key,
        }
    }

    /// Construct a new position from a FEN string
    pub fn from_fen(fen: &str) -> Result<Position, String> {
        from_fen(fen).map(|(grid, state)| Position::new(grid, state))
    }

    // Convert position to FEN representation
    pub fn to_fen(&self) -> String {
        to_fen(&self.grid, &self.state)
    }

    pub fn hash_key(&self) -> u64 {
        self.key
    }

    /// Get position non-positional state
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Get position position
    pub fn grid(&self) -> &[Piece; 64] {
        &self.grid
    }

    /// Get piece at square
    pub fn at(&self, sq: Square) -> Piece {
        unsafe { return *self.grid.get_unchecked(sq.to_usize()) }
    }

    pub fn king_square(&self, side: Side) -> Square {
        let pc = KING.pc(side);
        self.bb_pc(pc).bitscan()
    }

    fn put_piece(&mut self, pc: Piece, sq: Square) {
        debug_assert!(self.at(sq).is_none());

        let bb_mask = BB::new(sq);

        self.update_grid(sq, pc);

        debug_assert_eq!(self.bb_pc(pc) & bb_mask, EMPTY);
        debug_assert_eq!(self.bb_side(pc.side()) & bb_mask, EMPTY);

        unsafe {
            *self.bb_pieces.get_unchecked_mut(pc.to_usize()) ^= bb_mask;
            *self.bb_sides.get_unchecked_mut(pc.side().to_usize()) ^= bb_mask;
        }
    }

    fn remove_piece(&mut self, sq: Square) {
        debug_assert!(self.at(sq).is_some());

        let pc = self.at(sq);
        let bb_mask = BB::new(sq);

        self.update_grid(sq, NULL_PIECE);

        debug_assert_eq!(self.bb_pc(pc) & bb_mask, bb_mask);
        debug_assert_eq!(self.bb_side(pc.side()) & bb_mask, bb_mask);

        unsafe {
            *self.bb_pieces.get_unchecked_mut(pc.to_usize()) ^= bb_mask;
            *self.bb_sides.get_unchecked_mut(pc.side().to_usize()) ^= bb_mask;
        }
    }

    fn move_piece(&mut self, from: Square, to: Square) -> BB {
        debug_assert!(self.at(from).is_some());
        debug_assert!(self.at(to).is_none());

        let pc = self.at(from);
        let bb_mask = BB::new(from) | BB::new(to);

        debug_assert_eq!(self.bb_pc(pc) & BB::new(from), BB::new(from));
        debug_assert_eq!(self.bb_side(pc.side()) & BB::new(from), BB::new(from));
        debug_assert_eq!(self.bb_pc(pc) & BB::new(to), EMPTY);
        debug_assert_eq!(self.bb_side(pc.side()) & BB::new(to), EMPTY);

        self.update_grid(from, NULL_PIECE);
        self.update_grid(to, pc);

        unsafe {
            *self.bb_pieces.get_unchecked_mut(pc.to_usize()) ^= bb_mask;
            *self.bb_sides.get_unchecked_mut(pc.side().to_usize()) ^= bb_mask;
        }

        bb_mask
    }

    fn promote_piece(&mut self, sq: Square, new_pc: Piece) {
        let old_pc = self.at(sq);
        let bb_mask = BB::new(sq);

        debug_assert!(old_pc.is_some());
        debug_assert_eq!(old_pc.side(), new_pc.side());

        self.update_grid(sq, new_pc);

        debug_assert_eq!(self.bb_pc(old_pc) & bb_mask, bb_mask);
        debug_assert_eq!(self.bb_pc(new_pc) & bb_mask, EMPTY);
        debug_assert_eq!(self.bb_side(old_pc.side()) & bb_mask, bb_mask);

        unsafe {
            *(self.bb_pieces.get_unchecked_mut(old_pc.to_usize())) ^= bb_mask;
            *(self.bb_pieces.get_unchecked_mut(new_pc.to_usize())) |= bb_mask;
        }
    }

    /// Get bitboard of pieces for a particular side
    pub fn bb_side(&self, side: Side) -> BB {
        unsafe { return *self.bb_sides.get_unchecked(side.to_usize() & 1) }
    }

    /// Get bitboard of pieces for a particular piece
    pub fn bb_pc(&self, pc: Piece) -> BB {
        unsafe { return *self.bb_pieces.get_unchecked(pc.to_usize()) }
    }

    pub fn piece_iter(&self, pc: Piece) -> BBIterator {
        self.bb_pieces[pc.to_usize()].iter()
    }

    /// Get bitboard of sliding pieces for a particular side
    pub fn bb_sliders(&self, side: Side) -> (BB, BB) {
        let queens = self.bb_pc(QUEEN.pc(side));
        let rooks = self.bb_pc(ROOK.pc(side));
        let bishops = self.bb_pc(BISHOP.pc(side));
        (queens | bishops, queens | rooks)
    }

    /// Get bitboard of all occupied squares
    pub fn bb_occupied(&self) -> BB {
        self.bb_side(WHITE) | self.bb_side(BLACK)
    }

    /// Get bitboard of all empty squares
    pub fn bb_empty(&self) -> BB {
        !self.bb_occupied()
    }

    fn update_grid(&mut self, sq: Square, pc: Piece) {
        unsafe {
            *(self.grid.get_unchecked_mut(sq.to_usize())) = pc;
        }
    }

    pub fn piece_square_score(&self, piece_square_table: &PieceSquareTable) -> i16 {
        let mut score = 0i16;

        for (idx, &pc) in self.grid().iter().enumerate() {
            if pc.is_some() {
                let piece_square_value =
                    piece_square_table.score(pc.kind(), Square(idx).from_side(pc.side()));

                score += if self.state().stm == pc.side() {
                    piece_square_value
                } else {
                    -piece_square_value
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use unindent;

    #[test]
    fn test_is_not_too_big() {
        assert_eq!(std::mem::size_of::<Position>(), 224);
    }

    #[test]
    fn test_to_string() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").unwrap();

        let expected = unindent::unindent(
            "
          ABCDEFGH
        8|rnbqkbnr|8     side to move: white
        7|pppppppp|7  castling rights: QqKk
        6|........|6       en-passant: -
        5|........|5  half-move clock: 0
        4|........|4 full-move number: 1
        3|........|3              FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QqKk - 0 1
        2|PPPPPPPP|2              KEY: 0674AFC18BB45C18
        1|RNBQKBNR|1
          ABCDEFGH
        ",
        );
        assert_eq!(position.to_string(), expected);
    }
}
