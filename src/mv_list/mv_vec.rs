use bb::{BB, END_ROWS};
use castle::Castle;
use mv::Move;
use mv_list::MoveList;
use piece::*;
use square;
use square::Square;
use std;
use std::fmt;

/// MoveVec implements MoveList and collects moves in a vector.
/// Use `iter` to access the moves once they have been added.
#[derive(Clone)]
pub struct MoveVec {
    moves: Vec<Move>,
}

impl fmt::Display for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl MoveList for MoveVec {
    #[inline]
    fn add_moves(&mut self, from: Square, targets: BB, enemy: BB) {
        self.insert_moves(from, targets & (!enemy), Move::new_push);
        self.insert_moves(from, targets & enemy, Move::new_capture);
    }

    #[inline]
    fn add_castle(&mut self, castle: Castle) {
        self.moves.push(Move::new_castle(castle));
    }

    #[inline]
    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        self.moves.push(Move::new_ep_capture(from, to));
    }

    #[inline]
    fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, targets & END_ROWS, Move::new_promotion);
        self.insert_moves_by_shift(shift, targets & !END_ROWS, Move::new_push);
    }

    #[inline]
    fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, targets & END_ROWS, Move::new_capture_promotion);
        self.insert_moves_by_shift(shift, targets & !END_ROWS, Move::new_capture);
    }
}

impl MoveVec {
    #[inline]
    pub fn new() -> MoveVec {
        MoveVec {
            moves: Vec::with_capacity(60),
        }
    }

    pub fn to_string(&self) -> String {
        self.iter()
            .map(|mv: &Move| mv.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves.iter()
    }

    #[inline]
    fn insert_moves<F: Fn(Square, Square) -> Move>(&mut self, from: Square, targets: BB, f: F) {
        for (to, _) in targets.iter() {
            self.moves.push(f(from, to));
        }
    }

    #[inline]
    fn insert_moves_by_shift<F: Fn(Square, Square) -> Move>(
        &mut self,
        shift: usize,
        targets: BB,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as square::Internal);
            self.moves.push(f(from, to));
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    #[inline]
    fn insert_promos_by_shift<F: Fn(Square, Square, Kind) -> Move>(
        &mut self,
        shift: usize,
        targets: BB,
        f: F,
    ) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as square::Internal);
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
    use gen::*;
    use position::*;

    #[test]
    fn test_move_vec() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();

        legal_moves(&position, &mut list);

        assert_eq!(list.len(), 20);
    }
}
