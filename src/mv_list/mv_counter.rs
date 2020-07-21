use bb::{BB, END_ROWS};
use castle::Castle;
use mv_list::MoveList;
use square::Square;

/// MoveCounter implements MoveList and keeps a count of different types of moves added to it. It can count at most 256 moves since it uses `u8` internally
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveCounter {
    pub moves: u8,
    pub captures: u8,
    pub castles: u8,
    pub promotions: u8,
    pub ep_captures: u8,
}

impl MoveCounter {
    pub fn new() -> MoveCounter {
        MoveCounter {
            moves: 0,
            captures: 0,
            castles: 0,
            promotions: 0,
            ep_captures: 0,
        }
    }
}

impl MoveList for MoveCounter {
    fn add_moves(&mut self, _: Square, targets: BB, enemy: BB) {
        self.moves += targets.pop_count() as u8;
        self.captures += (targets & enemy).pop_count() as u8;
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
        self.moves += (targets & !END_ROWS).pop_count() as u8;

        let promo_count = (targets & END_ROWS).pop_count() * 4;
        self.moves += promo_count as u8;
        self.promotions += promo_count as u8;
    }

    fn add_pawn_captures(&mut self, _: usize, targets: BB) {
        // non-promotions
        let non_promo_count = (targets & !END_ROWS).pop_count();

        let promo_count = (targets & END_ROWS).pop_count() * 4;
        self.promotions += promo_count as u8;

        let total = promo_count + non_promo_count;
        self.moves += total as u8;
        self.captures += total as u8;
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
