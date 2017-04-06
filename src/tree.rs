use board::*;
use piece::Piece;
use square::*;
use mv_list::*;
use mv::Move;
use gen::legal_moves;

#[derive(Clone)]
pub struct Tree {
    board: Board,
    stack: Vec<StackElem>,
}

#[derive(Clone)]
struct StackElem {
    pub key: u64,
    pub captured: Option<(Piece, Square)>,
    pub state: State,
}

impl Tree {
    pub fn new(fen: &str) -> Tree {
        let board = Board::from_fen(fen).unwrap();

        Tree {
            board: board,
            stack: Vec::new(),
        }
    }

    pub fn count_legal_moves(&self) -> (bool, MoveCounter) {
        let mut move_counter = MoveCounter::new();
        let in_check = legal_moves(&self.board, &mut move_counter);

        (in_check, move_counter)
    }

    pub fn generate_legal_moves(&mut self) -> (bool, MoveVec) {
        let mut move_vec = MoveVec::new();
        let in_check = legal_moves(&self.board, &mut move_vec);
        (in_check, move_vec)
    }

    pub fn key(&self) -> u64 {
        self.board.hash_key()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn make(&mut self, mv: Move) {
        let before_state = self.board.state().clone();
        let key = self.board.hash_key();
        let capture = self.board.make(mv);

        self.stack.push(StackElem {
            key: key,
            captured: capture,
            state: before_state,
        })
    }

    pub fn unmake(&mut self, mv: Move) {
        let elem = self.stack.pop().unwrap();
        self.board.unmake(mv, elem.captured, &elem.state, elem.key);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key() {
        // Hash should not care what order moves are done in
        let mut tree = Tree::new(STARTING_POSITION_FEN);

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

        tree.unmake(mv_d);
        tree.unmake(mv_c);
        tree.unmake(mv_b);
        tree.unmake(mv_a);

        assert_eq!(tree.key(), key_init);

        tree.make(mv_c);
        tree.make(mv_b);
        tree.make(mv_a);
        tree.make(mv_d);

        assert_eq!(tree.key(), key_after_moves);
    }
}