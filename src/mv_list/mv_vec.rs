use crate::bb::{BB, END_ROWS};
use crate::castle::Castle;
use crate::mv::Move;
use crate::mv_list::MoveAdder;
use crate::piece::*;
use crate::square::{Square, SquareInternal};
use std;
use std::fmt;

/// MoveVec implements MoveAdder and collects moves in a vector.
/// Use `iter` to access the moves once they have been added.
#[derive(Clone)]
pub struct MoveVec {
    moves: Vec<Move>,
}

impl fmt::Display for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|mv: &Move| mv.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Debug for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|mv: &Move| mv.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl MoveAdder for MoveVec {
    fn add_captures(&mut self, from: Square, targets: BB) {
        self.insert_moves(from, targets, Move::new_capture);
    }

    fn add_non_captures(&mut self, from: Square, targets: BB) {
        self.insert_moves(from, targets, Move::new_push);
    }

    fn add_castle(&mut self, castle: Castle) {
        self.moves.push(Move::new_castle(castle));
    }

    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        self.moves.push(Move::new_ep_capture(from, to));
    }

    fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, targets & END_ROWS, Move::new_promotion);
        self.insert_moves_by_shift(shift, targets & !END_ROWS, Move::new_push);
    }

    fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, targets & END_ROWS, Move::new_capture_promotion);
        self.insert_moves_by_shift(shift, targets & !END_ROWS, Move::new_capture);
    }
}

impl Default for MoveVec {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveVec {
    pub fn new() -> MoveVec {
        MoveVec {
            moves: Vec::with_capacity(60),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves.iter()
    }

    fn insert_moves<F: Fn(Square, Square) -> Move>(&mut self, from: Square, targets: BB, f: F) {
        for (to, _) in targets.iter() {
            self.moves.push(f(from, to));
        }
    }

    fn insert_moves_by_shift<F: Fn(Square, Square) -> Move>(
        &mut self,
        shift: usize,
        targets: BB,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as SquareInternal);
            self.moves.push(f(from, to));
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn insert_promos_by_shift<F: Fn(Square, Square, Kind) -> Move>(
        &mut self,
        shift: usize,
        targets: BB,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as SquareInternal);
            self.moves.push(f(from, to, QUEEN));
            self.moves.push(f(from, to, KNIGHT));
            self.moves.push(f(from, to, BISHOP));
            self.moves.push(f(from, to, ROOK));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::generation::*;
    use crate::position::*;

    #[test]
    fn test_move_vec() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();

        legal_moves(&position, &mut list);

        assert_eq!(list.len(), 20);
    }
}
