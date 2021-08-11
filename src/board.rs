use mv::Move;
use piece::Piece;
use position::*;
use square::*;

pub struct Board {
    position: Position,
    stack: Vec<StackElem>,
}

#[derive(Clone)]
struct StackElem {
    pub key: u64,
    pub captured: Option<(Piece, Square)>,
    pub state: State,
    pub mv: Move,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(fen: &str) -> Board {
        let position = Position::from_fen(fen).unwrap();

        Board {
            position,
            stack: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn key(&self) -> u64 {
        self.position.hash_key()
    }

    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn make(&mut self, mv: Move) {
        let state = *self.position.state();
        let key = self.position.hash_key();
        let captured = self.position.make(mv);

        self.stack.push(StackElem {
            key,
            captured,
            state,
            mv,
        })
    }

    pub fn unmake(&mut self) {
        let elem = self.stack.pop().unwrap();
        self.position
            .unmake(elem.mv, elem.captured, &elem.state, elem.key);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key() {
        // Hash should not care what order moves are done in
        let mut tree = Board::new(STARTING_POSITION_FEN);

        let key_init = tree.key();

        let mv_a = Move::new_push(D2, D4);
        let mv_b = Move::new_push(G7, G5);
        let mv_c = Move::new_push(B1, A3);
        let mv_d = Move::new_push(G5, G4);

        tree.make(mv_a);
        tree.make(mv_b);
        tree.make(mv_c);
        tree.make(mv_d);

        let key_after_moves = tree.key();

        tree.unmake();
        tree.unmake();
        tree.unmake();
        tree.unmake();

        assert_eq!(tree.key(), key_init);

        tree.make(mv_c);
        tree.make(mv_b);
        tree.make(mv_a);
        tree.make(mv_d);

        assert_eq!(tree.key(), key_after_moves);
    }
}
