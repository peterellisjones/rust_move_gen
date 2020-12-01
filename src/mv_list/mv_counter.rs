use bb::{BB, END_ROWS};
use castle::Castle;
use mv_list::MoveList;
use square::Square;
use std::ops;

/// MoveCounter implements MoveList and keeps a count of different types of moves added to it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MoveCounter {
    pub moves: u64,
    pub captures: u64,
    pub castles: u64,
    pub promotions: u32,
    pub ep_captures: u32,
}

impl MoveCounter {
    pub fn new() -> MoveCounter {
        MoveCounter {
            ..Default::default()
        }
    }
}

impl MoveList for MoveCounter {
    fn add_captures(&mut self, _: Square, targets: BB) {
        let count = targets.pop_count() as u64;
        self.moves += count;
        self.captures += count;
    }

    fn add_non_captures(&mut self, _: Square, targets: BB) {
        self.moves += targets.pop_count() as u64;
    }

    fn add_castle(&mut self, _: Castle) {
        self.moves += 1;
        self.castles += 1;
    }

    fn add_pawn_ep_capture(&mut self, _: Square, _: Square) {
        self.moves += 1;
        self.captures += 1;
        self.ep_captures += 1;
    }

    fn add_pawn_pushes(&mut self, _: usize, targets: BB) {
        // non-promotions
        self.moves += (targets & !END_ROWS).pop_count() as u64;

        let promo_count = (targets & END_ROWS).pop_count() * 4;
        self.moves += promo_count as u64;
        self.promotions += promo_count;
    }

    fn add_pawn_captures(&mut self, _: usize, targets: BB) {
        // non-promotions
        let non_promo_count = (targets & !END_ROWS).pop_count();

        let promo_count = (targets & END_ROWS).pop_count() * 4;
        self.promotions += promo_count;

        let total = (promo_count + non_promo_count) as u64;
        self.moves += total;
        self.captures += total;
    }
}

impl ops::Add<MoveCounter> for MoveCounter {
    type Output = Self;

    fn add(self, other: MoveCounter) -> MoveCounter {
        MoveCounter {
            moves: self.moves + other.moves,
            captures: self.captures + other.captures,
            castles: self.castles + other.castles,
            promotions: self.promotions + other.promotions,
            ep_captures: self.ep_captures + other.ep_captures,
        }
    }
}

impl ops::AddAssign<MoveCounter> for MoveCounter {
    fn add_assign(&mut self, other: MoveCounter) {
        self.moves += other.moves;
        self.captures += other.captures;
        self.castles += other.castles;
        self.promotions += other.promotions;
        self.ep_captures += other.ep_captures;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gen::*;
    use position::*;

    #[test]
    fn test_move_counter() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut counter = MoveCounter::new();

        legal_moves(&position, &mut counter);

        assert_eq!(counter.moves, 20);
    }
}
