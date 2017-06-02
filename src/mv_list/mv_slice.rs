use mv::{Move,NULL_MOVE};
use square::Square;
use square;
use bb::{BB, END_ROWS};
use castle::Castle;
use piece::*;
use std::fmt;
use mv_list::MoveList;

/// MoveCounter implements MoveList and collects moves in a vector.
/// Use `iter` to access the moves once they have been added.
pub struct MoveSlice<'a> {
    moves: &'a mut [Move],
    len: usize,
}

impl<'a> fmt::Display for MoveSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a> fmt::Debug for MoveSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a> MoveList for MoveSlice<'a> {
    #[inline]
    fn add_moves(&mut self, from: Square, targets: BB, enemy: BB) {
        self.insert_moves(from, targets & (!enemy), Move::new_push);
        self.insert_moves(from, targets & enemy, Move::new_capture);
    }

    #[inline]
    fn add_castle(&mut self, castle: Castle) {
        self.push(Move::new_castle(castle));
    }

    #[inline]
    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        self.push(Move::new_ep_capture(from, to));
    }

    #[inline]
    fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, (targets & END_ROWS), Move::new_promotion);
        self.insert_moves_by_shift(shift, (targets & !END_ROWS), Move::new_push);
    }

    #[inline]
    fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, (targets & END_ROWS), Move::new_capture_promotion);
        self.insert_moves_by_shift(shift, (targets & !END_ROWS), Move::new_capture);
    }
}

impl<'a> MoveSlice<'a> {
    #[inline]
    pub fn new(moves: &'a mut [Move]) -> MoveSlice<'a> {
        MoveSlice { moves: moves, len: 0 }
    }

    #[inline]
    pub fn clear(&mut self) {
      self.len = 0;
    }

    #[inline]
    fn push(&mut self, mv: Move) {
          let idx = self.len;
          self.moves[idx] = mv;
          self.len += 1;
    }

    #[inline]
    fn insert_moves<F: Fn(Square, Square) -> Move>(&mut self, from: Square, targets: BB, f: F) {
        for (to, _) in targets.iter() {
            self.push(f(from, to));
        }
    }

    #[inline]
    fn insert_moves_by_shift<F: Fn(Square, Square) -> Move>(&mut self,
                                                            shift: usize,
                                                            targets: BB,
                                                            f: F) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as square::Internal);
            let idx = self.len;
            self.moves[idx] = f(from, to);
            self.len += 1;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn insert_promos_by_shift<F: Fn(Square, Square, Kind) -> Move>(&mut self,
                                                                   shift: usize,
                                                                   targets: BB,
                                                                   f: F) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift as square::Internal);
            self.push(f(from, to, QUEEN));
            self.push(f(from, to, KNIGHT));
            self.push(f(from, to, BISHOP));
            self.push(f(from, to, ROOK));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use position::*;
    use gen::*;

    #[test]
    fn test_move_slice() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();

        let mut arr = [NULL_MOVE; 256];
        let mut move_slice = MoveSlice::new(&mut arr);

        legal_moves(&position, &mut move_slice);

        assert_eq!(move_slice.len(), 20);
    }
}
