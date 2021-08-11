use super::piece_square_table::PieceSquareTable;
use bb::{BB, EMPTY, END_ROWS};
use castle::Castle;
use mv::{Move, MoveScore};
use mv_list::MoveAdder;
use piece::*;
use side::Side;
use square;
use square::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

/// SortedMoveAdder collects moves including the piece-square score for making the move
/// Moves are sorted by an 'ordering score' and stored in a provided SortedMoveHeap
///     
/// Non-captures are ordered by priotising destination squares that are
/// further up the board relative to the mover, with a bonus for pawns
/// near promotion, a handicap for non-pawn moves to rank 1, and a handicap
/// for pawn moves to low ranks.
///
/// Captures are ordered by most-valuable-victim / least-valuable-aggresor
/// with piece scores worth:
///     Pawn: 0
///     King: 1 (can only be aggresor)
///     Knight: 2
///     Bishop: 3
///     Rook: 4
///     Queen: 5
///
/// Move ordering priorities:
/// -250: promotion to bishop
/// -200: promotion to rook
/// -145..-145: capture and promotion to bishop (exact score based on MVV-LVA)
/// -105..-95: capture and promotion to rook (exact score based on MVV-LVA)
/// 0: non-pawn to rank 1
/// 1: -
/// 2: pawn to rank 3
/// 3: pawn to rank 4, non-pawn to rank 2
/// 4: pawn to rank 5, non-pawn to rank 3
/// 5: non-pawn to rank 4
/// 6: non-pawn to rank 5
/// 7: non-pawn to rank 6
/// 8: non-pawn to rank 7
/// 9: non-pawn to rank 8
/// 10: pawn to rank 6
/// 11: pawn to rank 7
/// 20: castle
/// 25: promotion to knight
/// 50: promotion to queen
/// 95..105: capture (exact score based on MVV-LVA)
/// 120..130: capture and promotion to knight (exact score based on MVV-LVA)
/// 145..155: capture and promotion to queen (exact score based on MVV-LVA)
pub struct SortedMoveAdder<'a> {
    moves: &'a mut SortedMoveHeap,
    piece_square_table: &'a PieceSquareTable,
    piece_grid: &'a [Piece; 64],
    stm: Side,
}

impl<'a> fmt::Display for SortedMoveAdder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.moves
                .0
                .iter()
                .map(|item: &SortedMoveHeapItem| item.move_score().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<'a> fmt::Debug for SortedMoveAdder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.moves
                .0
                .iter()
                .map(|item: &SortedMoveHeapItem| item.move_score().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<'a> MoveAdder for SortedMoveAdder<'a> {
    fn add_captures(&mut self, from: Square, targets: BB) {
        if targets == EMPTY {
            return;
        }

        let stm = self.stm;

        let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
        let from_score = self
            .piece_square_table
            .score(from_kind, from.from_side(stm));

        for (to, _) in targets.iter() {
            let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

            let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
            let capture_score = self
                .piece_square_table
                .score(capture_kind, to.from_side(stm.flip()));

            self.insert(
                Move::new_capture(from, to),
                -from_score + to_score + capture_score,
                self.capture_ordering_score(from_kind, capture_kind),
            );
        }
    }

    fn add_non_captures(&mut self, from: Square, targets: BB) {
        if targets == EMPTY {
            return;
        }

        let stm = self.stm;

        let from_kind = unsafe { self.piece_grid.get_unchecked(from.to_usize()).kind() };
        let from_score = self
            .piece_square_table
            .score(from_kind, from.from_side(stm));

        for (to, _) in targets.iter() {
            let relative_to_sq = to.from_side(stm);
            let to_score = self.piece_square_table.score(from_kind, relative_to_sq);

            self.insert(
                Move::new_push(from, to),
                -from_score + to_score,
                self.non_capture_ordering_score(relative_to_sq),
            );
        }
    }

    fn add_castle(&mut self, castle: Castle) {
        let score = self.piece_square_table.castle_score(castle);
        self.insert(
            Move::new_castle(castle),
            score,
            self.castle_ordering_score(),
        );
    }

    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        let stm = self.stm;
        let from_score = self.piece_square_table.score(PAWN, from.from_side(stm));
        let to_score = self.piece_square_table.score(PAWN, to.from_side(stm));
        // capture square is the file of the 'to' square
        // and the rank of the 'from' square
        let capture_sq = from.along_row_with_col(to);

        let capture_score = self
            .piece_square_table
            .score(PAWN, capture_sq.from_side(stm.flip()));

        let score = -from_score + to_score + capture_score;

        self.insert(
            Move::new_ep_capture(from, to),
            score,
            self.capture_ordering_score(PAWN, PAWN),
        );
    }

    fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
        let stm = self.stm;
        let from_kind = PAWN;

        for (to, _) in (targets & !END_ROWS).iter() {
            let from = to.rotate_right(shift as square::Internal);
            let from_score = self
                .piece_square_table
                .score(from_kind, from.from_side(stm));
            let relative_to_sq = to.from_side(stm);
            let to_score = self.piece_square_table.score(from_kind, relative_to_sq);

            let move_ordering_score = self.pawn_non_capture_ordering_score(relative_to_sq);

            self.insert(
                Move::new_push(from, to),
                -from_score + to_score,
                move_ordering_score,
            );
        }

        for (to, _) in (targets & END_ROWS).iter() {
            let from = to.rotate_right(shift as square::Internal);
            let from_score = self
                .piece_square_table
                .score(from_kind, from.from_side(stm));

            for (to_kind, move_ordering_score) in &Self::PROMOTION_ORDERING_SCORES {
                let to_score = self.piece_square_table.score(*to_kind, to.from_side(stm));

                self.insert(
                    Move::new_promotion(from, to, *to_kind),
                    -from_score + to_score,
                    *move_ordering_score,
                );
            }
        }
    }

    fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
        let stm = self.stm;
        let from_kind = PAWN;

        for (to, _) in (targets & !END_ROWS).iter() {
            let from = to.rotate_right(shift as square::Internal);
            let from_score = self
                .piece_square_table
                .score(from_kind, from.from_side(stm));
            let to_score = self.piece_square_table.score(from_kind, to.from_side(stm));

            let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
            let capture_score = self
                .piece_square_table
                .score(capture_kind, to.from_side(stm.flip()));

            let move_ordering_score = self.capture_ordering_score(PAWN, capture_kind);

            self.insert(
                Move::new_capture(from, to),
                -from_score + to_score + capture_score,
                move_ordering_score,
            );
        }

        for (to, _) in (targets & END_ROWS).iter() {
            let from = to.rotate_right(shift as square::Internal);
            let from_score = self
                .piece_square_table
                .score(from_kind, from.from_side(stm));

            let capture_kind = unsafe { self.piece_grid.get_unchecked(to.to_usize()).kind() };
            let capture_score = self
                .piece_square_table
                .score(capture_kind, to.from_side(stm.flip()));

            for (to_kind, promo_move_ordering_score) in &Self::PROMOTION_ORDERING_SCORES {
                let to_score = self.piece_square_table.score(*to_kind, to.from_side(stm));

                let move_ordering_score = 100i16 + promo_move_ordering_score;

                self.insert(
                    Move::new_capture_promotion(from, to, *to_kind),
                    -from_score + to_score + capture_score,
                    move_ordering_score,
                );
            }
        }
    }
}

impl<'a> SortedMoveAdder<'a> {
    // Maps promotion target pieces to base move ordering score
    // reason: calculating rook and bishop promotions almost never gives
    // an advantage over queen and knight promotions
    const PROMOTION_ORDERING_SCORES: [(Kind, i16); 4] = [
        (QUEEN, 50i16),
        (KNIGHT, 25i16),
        (ROOK, -200i16),
        (BISHOP, -250i16),
    ];
    pub fn new(
        piece_square_table: &'a PieceSquareTable,
        piece_grid: &'a [Piece; 64],
        stm: Side,
        moves: &'a mut SortedMoveHeap,
    ) -> SortedMoveAdder<'a> {
        SortedMoveAdder {
            moves,
            piece_square_table,
            piece_grid,
            stm,
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn insert(&mut self, mv: Move, piece_square_score: i16, move_ordering_score: i16) {
        let item =
            SortedMoveHeapItem((MoveScore::new(mv, piece_square_score), move_ordering_score));

        self.moves.push(item);
    }

    // Captures are order based on
    // most-valuable-victim - least-valuable-attacker
    fn capture_ordering_score(&self, mover_kind: Kind, capture_kind: Kind) -> i16 {
        debug_assert_ne!(capture_kind, KING);
        100i16 + self.mvv_score(capture_kind) - self.mvv_score(mover_kind)
    }

    fn mvv_score(&self, kind: Kind) -> i16 {
        const MVV_SCORE: [i16; 6] = [
            3, // Bishop
            5, // Queen
            4, // Rook
            2, // Knight
            0, // Pawn
            1, // King
        ];
        MVV_SCORE[kind.to_usize()]
    }

    fn non_capture_ordering_score(&self, relative_to_sq: Square) -> i16 {
        let row = relative_to_sq.row();
        // no bonus for moves to rank 1
        if row == 0 {
            return 0i16;
        }
        // +2 bonus compared to pawn moves
        2i16 + (row as i16)
    }

    fn pawn_non_capture_ordering_score(&self, relative_to_sq: Square) -> i16 {
        let row = relative_to_sq.row();

        if row > 4 {
            // bonus for moves to rank 6, 7, 8 (=row 5, 6, 7)
            return row as i16 + 5i16;
        }

        row as i16
    }

    fn castle_ordering_score(&self) -> i16 {
        20i16
    }
}

pub struct SortedMoveHeapItem((MoveScore, i16));

impl SortedMoveHeapItem {
    fn ordering_score(&self) -> &i16 {
        &self.0 .1
    }

    pub fn move_score(&self) -> &MoveScore {
        &self.0 .0
    }
}

impl PartialEq for SortedMoveHeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.ordering_score() == other.ordering_score()
    }
}

impl Eq for SortedMoveHeapItem {}

impl PartialOrd for SortedMoveHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ordering_score().partial_cmp(other.ordering_score())
    }
}

impl Ord for SortedMoveHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering_score().cmp(other.ordering_score())
    }
}

pub struct SortedMoveHeap(BinaryHeap<SortedMoveHeapItem>);

impl SortedMoveHeap {
    pub fn new(capacity: usize) -> SortedMoveHeap {
        SortedMoveHeap(BinaryHeap::<SortedMoveHeapItem>::with_capacity(capacity))
    }

    pub fn peek(&self) -> Option<&SortedMoveHeapItem> {
        self.0.peek()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, elem: SortedMoveHeapItem) {
        self.0.push(elem);
    }

    pub fn iter(self) -> std::collections::binary_heap::IntoIterSorted<SortedMoveHeapItem> {
        self.0.into_iter_sorted()
    }

    pub fn into_sorted_vec(self) -> Vec<MoveScore> {
        self.iter()
            .map(|e| *e.move_score())
            .collect::<Vec<MoveScore>>()
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use gen::*;
    use position::*;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use side::WHITE;

    #[test]
    fn test_scored_move_vec() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let piece_square_table = PieceSquareTable::new([[100i16; 64]; 6]);

        let mut heap = SortedMoveHeap::new(32);
        let mut list = SortedMoveAdder::new(
            &piece_square_table,
            position.grid(),
            position.state().stm,
            &mut heap,
        );

        legal_moves(&position, &mut list);

        assert_eq!(heap.len(), 20);
    }

    #[test]
    fn test_pawn_push_scoring() {
        let position = &Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut piece_square_values = [[100i16; 64]; 6];

        // make Pawn C2 150, Pawn C4 165. Should have move worth +15
        piece_square_values[PAWN.to_usize()][C2.to_usize()] = 150;
        piece_square_values[PAWN.to_usize()][C4.to_usize()] = 165;

        let piece_square_table = PieceSquareTable::new(piece_square_values);
        let mut heap = SortedMoveHeap::new(32);
        let mut list = SortedMoveAdder::new(
            &piece_square_table,
            position.grid(),
            position.state().stm,
            &mut heap,
        );

        legal_moves(&position, &mut list);

        assert_list_includes_moves(heap, &["c2c4 (15)"]);
    }

    #[test]
    fn test_push_scoring() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b QqKk - 0 1")
                .unwrap();
        let mut piece_square_values = [[100i16; 64]; 6];

        // make knight B1 300, C3 333. Should have move worth +33
        piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
        piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

        let piece_square_table = PieceSquareTable::new(piece_square_values);
        let mut heap = SortedMoveHeap::new(32);
        let mut list = SortedMoveAdder::new(
            &piece_square_table,
            position.grid(),
            position.state().stm,
            &mut heap,
        );

        legal_moves(&position, &mut list);

        assert_list_includes_moves(heap, &["b8c6 (33)"]);
    }
    #[test]
    fn test_capture_scoring() {
        // white pawn on C3
        let position =
            &Position::from_fen("rnbqkbnr/pppppppp/2P5/8/8/8/PP1PPPPP/RNBQKBNR b QqKk - 0 1")
                .unwrap();
        let mut piece_square_values = [[100i16; 64]; 6];

        // make knight B1 300, C3 333. Should have move worth +33
        piece_square_values[KNIGHT.to_usize()][B1.to_usize()] = 300;
        piece_square_values[KNIGHT.to_usize()][C3.to_usize()] = 333;

        // pawn on C6 worth 50
        piece_square_values[PAWN.to_usize()][C6.to_usize()] = 50;

        let piece_square_table = PieceSquareTable::new(piece_square_values);
        let mut heap = SortedMoveHeap::new(32);
        let mut list = SortedMoveAdder::new(
            &piece_square_table,
            position.grid(),
            position.state().stm,
            &mut heap,
        );

        legal_moves(&position, &mut list);

        assert_list_includes_moves(heap, &["b8xc6 (83)"]);
    }

    #[test]
    fn test_integrity() {
        // makes 30 random moves, and checks that adding the move scores adds up to the
        // final position score
        let position = &mut Position::from_fen(STARTING_POSITION_FEN).unwrap();
        let piece_square_table = random_piece_square_table();

        let initial_score = score_for_position(position, &piece_square_table);

        let mut moves_scores = 0i16;

        for _ in 0..30 {
            // note must be even number of iterations to ensure final score is calculated from WHITE's PoV
            let stm = position.state().stm;

            let move_score = {
                let mut heap = SortedMoveHeap::new(32);
                let mut list = SortedMoveAdder::new(
                    &piece_square_table,
                    position.grid(),
                    position.state().stm,
                    &mut heap,
                );

                legal_moves(&position, &mut list);

                let sorted_vec = heap.into_sorted_vec();

                *sorted_vec.choose(&mut rand::thread_rng()).unwrap()
            };

            let score = move_score.score();
            let mv = move_score.mv();

            moves_scores += if stm == WHITE { score } else { -score };

            position.make(mv);
        }

        let score_after_moves = score_for_position(position, &piece_square_table);

        assert_eq!(initial_score + moves_scores, score_after_moves);
    }

    fn assert_list_includes_moves(heap: SortedMoveHeap, moves: &[&'static str]) {
        let sorted_vec = heap.into_sorted_vec();
        for &m in moves.iter() {
            assert!(sorted_vec
                .iter()
                .map(|move_score| format!("{} ({})", move_score.mv(), move_score.score()))
                .any(|mv| mv == m));
        }
    }

    fn random_piece_square_table() -> PieceSquareTable {
        let mut piece_square_values = [[100i16; 64]; 6];

        for pc in 0..6 {
            for sq in 0..64 {
                piece_square_values[pc][sq] = rand::thread_rng().gen_range(100, 200) as i16;
            }
        }

        PieceSquareTable::new(piece_square_values)
    }

    fn score_for_position(pos: &Position, piece_square_table: &PieceSquareTable) -> i16 {
        let mut score = 0i16;

        for (idx, &pc) in pos.grid().iter().enumerate() {
            if pc.is_some() {
                let piece_square_value =
                    piece_square_table.score(pc.kind(), Square(idx).from_side(pc.side()));

                score += if pos.state().stm == pc.side() {
                    piece_square_value
                } else {
                    -piece_square_value
                }
            }
        }

        score
    }
}
