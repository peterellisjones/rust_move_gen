use rand;
use rand::Rng;
use piece::*;
use square::Square;
use square;
use castle::*;
use position::State;
use side::*;

/// Zobrist represents a set of keys for Zobrist hashing
#[derive(Debug)]
pub struct Zobrist {
    pieces: [u64; 12], // rotated based on square
    castling_rights: [u64; 16],
    en_passant_file: [u64; 8],
    castles: [[u64; 2]; 2], // pre-computed based on piece-square changes
    stm: u64,
}

/// Default zobrist_keys generated with seed = 1
pub static DEFAULT_ZOBRISH_HASH: Zobrist = Zobrist {
    pieces: [16257666806172921645,
             12079090740189436754,
             11577349684372483860,
             8265070477432972399,
             17204147743807346879,
             10840387247671765879,
             11023604230088064055,
             15372782004648025408,
             17607845492419163657,
             4820222721347483354,
             9222096121752829227,
             10997107696558716930],

    castling_rights: [5901611952838449075,
                      16928860654864062033,
                      3006969943347880664,
                      16761043879460025667,
                      15332107909825879061,
                      1522114280938701486,
                      1327047711097840467,
                      7301561155728042398,
                      4479697827097181280,
                      2468172810615015476,
                      11492078287679521532,
                      11685917599786030246,
                      10403772991926020454,
                      17478376828681188621,
                      15580394547059712216,
                      4575347809368850956],

    en_passant_file: [3348961424409688254,
                      2427135436213657123,
                      2898060206792371384,
                      14683683615540271254,
                      952321900370658987,
                      5796203641919266782,
                      9554333785051357809,
                      12082317543310182802],

    castles: [[10993434298710260570, 16775444492128222288],
              [13399088802984349794, 13139464958289927509]],

    stm: 5703255076737973876,
};

impl Zobrist {
    #[allow(dead_code)]
    fn new(seed: usize) -> Zobrist {
        let seed_arr: &[usize] = &[seed];
        let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed_arr);

        let mut keys = Zobrist {
            pieces: [0; 12],
            castling_rights: [0; 16],
            castles: [[0; 2]; 2],
            en_passant_file: [0; 8],
            stm: 0,
        };

        for pc in 0..12 {
            keys.pieces[pc] = rng.next_u64();
        }

        for cr in 0..16 {
            keys.castling_rights[cr] = rng.next_u64();
        }

        for ep in 0..8 {
            keys.en_passant_file[ep] = rng.next_u64();
        }

        keys.stm = rng.next_u64();

        // pre-compute castles
        for &side in &[WHITE, BLACK] {
            for &castle in &[QUEEN_SIDE, KING_SIDE] {
                let mut hash = 0u64;

                let (king_from, king_to) = castle_king_squares(side, castle);
                hash ^= keys.piece_square(KING.pc(side), king_from);
                hash ^= keys.piece_square(KING.pc(side), king_to);

                let (rook_from, rook_to) = castle_rook_squares(side, castle);
                hash ^= keys.piece_square(ROOK.pc(side), rook_to);
                hash ^= keys.piece_square(ROOK.pc(side), rook_from);

                keys.castles[castle.to_usize()][side.to_usize()] = hash;
            }
        }

        keys
    }

    /// Generates the hash of the entire position
    pub fn position(&self, grid: &[Piece; 64], state: &State) -> u64 {
        let mut hash = 0u64;

        for (idx, &pc) in grid.iter().enumerate().filter(|&(_, &pc)| pc.is_some()) {
            hash ^= self.piece_square(pc, Square::new(idx as square::Internal));
        }

        hash ^= self.castling_rights[state.castling_rights.to_usize()];

        hash ^= self.ep_hash(state.ep_square);

        hash
    }

    /// Generates the difference between two states
    #[inline]
    pub fn state(&self, before: &State, after: &State) -> u64 {
        self.castling_rights[before.castling_rights.to_usize()] ^
        self.castling_rights[after.castling_rights.to_usize()] ^
        self.ep_hash(before.ep_square) ^ self.ep_hash(after.ep_square) ^ self.stm
    }

    #[inline]
    pub fn castle(&self, castle: Castle, stm: Side) -> u64 {
        self.castles[castle.to_usize()][stm.to_usize()]
    }

    #[inline]
    pub fn capture(&self, captured: Piece, capture_square: Square) -> u64 {
        self.piece_square(captured, capture_square)
    }

    #[inline]
    pub fn push(&self, pc_from: Piece, from: Square, pc_to: Piece, to: Square) -> u64 {
        self.piece_square(pc_from, from) ^ self.piece_square(pc_to, to)
    }

    #[inline]
    fn piece_square(&self, piece: Piece, square: Square) -> u64 {
        self.pieces[piece.to_usize()].rotate_left(square.to_u32())
    }

    #[inline]
    fn ep_hash(&self, ep_square: Option<Square>) -> u64 {
        if ep_square.is_some() {
            self.en_passant_file[ep_square.unwrap().col() as usize]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use square::*;

    #[test]
    fn test_castle() {
        let keys = Zobrist::new(1);

        let side = WHITE;
        let castle = QUEEN_SIDE;

        let actual_key = keys.castle(castle, side);

        let mut expected_key = 0u64;
        expected_key ^= keys.piece_square(WHITE_KING, E1);
        expected_key ^= keys.piece_square(WHITE_KING, C1);
        expected_key ^= keys.piece_square(WHITE_ROOK, A1);
        expected_key ^= keys.piece_square(WHITE_ROOK, D1);

        assert_eq!(actual_key, expected_key);
    }
}
