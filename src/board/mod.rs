pub mod fen;
pub mod make;

use std::fmt;
use square::*;
use side::Side;
use piece::*;
use bb::*;
use castling_rights::*;
use side::*;
use super::util::grid_to_string_with_props;
use self::fen::*;
use hash::{Zobrist, DEFAULT_ZOBRISH_HASH};

use std;

pub const STARTING_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w \
                                                 QqKk - 0 1";

/// State encodes all game state except position
#[derive(Debug, Clone)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,
    pub stm: Side,
    pub full_move_number: usize,
    pub half_move_clock: usize,
}

/// Board encodes all positional information and non-positional game state
pub struct Board {
    // grid is an array representation of board positions
    grid: [Option<Piece>; 64],
    // bb_sides represents a bitboard for each side
    bb_sides: [BB; 2],
    // bb_pieces represents a bitboard for each piece
    bb_pieces: [BB; 12],
    // state represents non-positional game state (eg side to move)
    state: State,

    key: u64,
    hash: &'static Zobrist,
}

impl std::clone::Clone for Board {
    fn clone(&self) -> Self {
        Board {
            grid: self.grid,
            bb_sides: self.bb_sides.clone(),
            bb_pieces: self.bb_pieces.clone(),
            state: self.state.clone(),
            key: self.key,
            hash: self.hash,
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Board {
    pub fn new(grid: [Option<Piece>; 64], state: State) -> Board {
        let mut bb_pieces = [EMPTY; 12];
        let mut bb_sides = [EMPTY; 2];

        for (idx, pc_opt) in grid.iter().enumerate().filter(|&(_, &pc)| pc.is_some()) {
            let pc = pc_opt.unwrap();
            let bb_mask = BB::new(Square::new(idx));
            bb_sides[pc.side().raw()] |= bb_mask;
            bb_pieces[pc.to_usize()] |= bb_mask;
        }

        let hash = &DEFAULT_ZOBRISH_HASH;
        let key = hash.board(&grid, &state);

        Board {
            grid: grid,
            bb_pieces: bb_pieces,
            bb_sides: bb_sides,
            state: state,
            hash: &DEFAULT_ZOBRISH_HASH,
            key: key,
        }
    }

    /// Construct a new board from a FEN string
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        from_fen(fen).map(|(grid, state)| Board::new(grid, state))
    }

    // Convert board to FEN representation
    pub fn to_fen(&self) -> String {
        to_fen(&self.grid, &self.state)
    }

    #[inline]
    pub fn hash_key(&self) -> u64 {
        self.key
    }

    /// Get board non-positional state
    #[inline]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Get board position
    #[inline]
    pub fn grid(&self) -> &[Option<Piece>; 64] {
        &self.grid
    }

    /// Get piece at square
    #[inline]
    pub fn at(&self, sq: Square) -> Option<Piece> {
        unsafe { return *self.grid.get_unchecked(sq.to_usize()) }
    }

    pub fn to_string(&self) -> String {
        let props = vec![("    side to move", self.state.stm.to_string()),
                         (" castling rights", self.state.castling_rights.to_string()),
                         ("      en-passant",
                          self.state.ep_square.map_or("-".to_string(), |s| s.to_string())),
                         (" half-move clock", self.state.half_move_clock.to_string()),
                         ("full-move number", self.state.full_move_number.to_string()),
                         ("             FEN", self.to_fen())];
        grid_to_string_with_props(|sq: Square| -> char { self.at(sq).map_or('.', |sq| sq.to_char()) },
                                  props.as_slice())
    }

    #[inline]
    fn put_piece(&mut self, pc: Piece, sq: Square) {
        debug_assert!(self.at(sq).is_none());
        self.grid[sq.to_usize()] = Some(pc);
        let bb_mask = BB::new(sq);
        let idx = pc.to_usize();
        self.bb_sides[idx & 1] |= bb_mask;
        self.bb_pieces[idx] |= bb_mask;
    }

    #[inline]
    fn remove_piece(&mut self, sq: Square) {
        debug_assert!(self.at(sq).is_some());
        let pc = self.grid[sq.to_usize()].unwrap();
        self.grid[sq.to_usize()] = None;
        let bb_mask = !BB::new(sq);
        let idx = pc.to_usize();
        self.bb_sides[idx & 1] &= bb_mask;
        self.bb_pieces[idx] &= bb_mask;
    }

    #[inline]
    fn move_piece(&mut self, from: Square, to: Square) -> BB {
        debug_assert!(self.at(from).is_some());
        debug_assert!(self.at(to).is_none());
        let pc = self.grid[from.to_usize()].unwrap();

        self.grid[from.to_usize()] = None;
        self.grid[to.to_usize()] = Some(pc);

        let bb_mask = BB::new(from) | BB::new(to);
        let idx = pc.to_usize();
        self.bb_sides[idx & 1] ^= bb_mask;
        self.bb_pieces[idx] ^= bb_mask;

        bb_mask
    }

    fn change_piece(&mut self, sq: Square, new_pc: Piece) {
        debug_assert!(self.at(sq).is_some());
        let old_pc = self.at(sq).unwrap();
        self.grid[sq.to_usize()] = Some(new_pc);

        let bb_mask = BB::new(sq);
        self.bb_pieces[old_pc.to_usize()] ^= bb_mask;
        self.bb_pieces[new_pc.to_usize()] |= bb_mask;
    }

    /// Get bitboard of pieces for a particular side
    pub fn bb_side(&self, side: Side) -> BB {
        unsafe { return *self.bb_sides.get_unchecked(side.to_usize() & 1) }
    }

    /// Get bitboard of pieces for a particular piece
    pub fn bb_pc(&self, pc: Piece) -> BB {
        unsafe { return *self.bb_pieces.get_unchecked(pc.to_usize()) }
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
}

#[cfg(test)]
mod test {
    use super::*;
    use unindent;

    #[test]
    fn test_to_string() {
        let board = &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").unwrap();

        let expected = unindent::unindent("
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
        ");
        assert_eq!(board.to_string(), expected);
    }
}
