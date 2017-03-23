use side::Side;
use std::fmt;
use castle::*;

type Internal = usize;

/// Represents the right to castle in a particular director for a particular side
#[derive(PartialEq, Copy, Clone)]
pub struct CastlingRights(pub Internal);

pub const WHITE_QS: CastlingRights = CastlingRights(1);
pub const BLACK_QS: CastlingRights = CastlingRights(2);
pub const WHITE_KS: CastlingRights = CastlingRights(4);
pub const BLACK_KS: CastlingRights = CastlingRights(8);

pub const NO_RIGHTS: CastlingRights = CastlingRights(0);
pub const ALL_RIGHTS: CastlingRights = CastlingRights(1 | 2 | 4 | 8);
pub const WHITE_RIGHTS: CastlingRights = CastlingRights(1 | 4);

const CASTLING_RIGHTS_CHARS: [char; 4] = ['Q', 'q', 'K', 'k'];

impl CastlingRights {
    pub fn to_usize(&self) -> Internal {
        self.0
    }

    pub fn has(&self, right: CastlingRights) -> bool {
        self.0 & right.0 != 0
    }

    #[allow(dead_code)]
    pub fn add(&mut self, castle: Castle, side: Side) {
        self.0 |= CastlingRights::from(castle, side).to_usize()
    }

    pub fn set(&mut self, rights: CastlingRights) {
        self.0 |= rights.0;
    }

    pub fn clear_side(&mut self, side: Side) {
        let rights = WHITE_RIGHTS.0 << side.raw();
        self.0 &= !rights;
    }

    pub fn clear(&mut self, rights: CastlingRights) {
        self.0 &= !rights.0;
    }

    #[allow(dead_code)]
    pub fn side_can(&self, side: Side) -> bool {
        let rights = WHITE_RIGHTS.0 << side.raw();
        self.0 & rights != 0
    }

    pub fn any(&self) -> bool {
        self.0 != 0
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        if !self.any() {
            return "-".to_string();
        }

        for i in 0..4 {
            let right = CastlingRights(1 << i);
            if self.has(right) {
                string.push(CASTLING_RIGHTS_CHARS[i]);
            }
        }

        string
    }

    pub fn from(castle: Castle, side: Side) -> CastlingRights {

        CastlingRights(1 << (castle.to_usize() * 2 + side.0))
    }

    pub fn parse(chr: char) -> Result<CastlingRights, String> {
        for i in 0..4 {
            if CASTLING_RIGHTS_CHARS[i] == chr {
                return Ok(CastlingRights(1 << i));
            }
        }
        Err(format!("Invalid castle: {}", chr))
    }
}


impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use side::*;

    #[test]
    fn can_1() {
        let rights = WHITE_QS;
        assert_eq!(rights.has(WHITE_QS), true);
        assert_eq!(rights.has(BLACK_QS), false);
        assert_eq!(rights.has(WHITE_KS), false);
        assert_eq!(rights.has(BLACK_KS), false);
    }

    #[test]
    fn can_2() {
        let rights = BLACK_QS;
        assert_eq!(rights.has(WHITE_QS), false);
        assert_eq!(rights.has(BLACK_QS), true);
        assert_eq!(rights.has(WHITE_KS), false);
        assert_eq!(rights.has(BLACK_KS), false);
    }

    #[test]
    fn can_3() {
        let rights = WHITE_KS;
        assert_eq!(rights.has(WHITE_QS), false);
        assert_eq!(rights.has(BLACK_QS), false);
        assert_eq!(rights.has(WHITE_KS), true);
        assert_eq!(rights.has(BLACK_KS), false);
    }

    #[test]
    fn can_4() {
        let rights = BLACK_KS;
        assert_eq!(rights.has(WHITE_QS), false);
        assert_eq!(rights.has(BLACK_QS), false);
        assert_eq!(rights.has(WHITE_KS), false);
        assert_eq!(rights.has(BLACK_KS), true);
    }

    #[test]
    fn can_5() {
        let mut rights = BLACK_KS;
        rights.add(QUEEN_SIDE, WHITE);
        assert_eq!(rights.has(WHITE_QS), true);
        assert_eq!(rights.has(BLACK_QS), false);
        assert_eq!(rights.has(WHITE_KS), false);
        assert_eq!(rights.has(BLACK_KS), true);
    }

    #[test]
    fn can_6() {
        assert_eq!(NO_RIGHTS.has(WHITE_QS), false);
        assert_eq!(NO_RIGHTS.has(BLACK_QS), false);
        assert_eq!(NO_RIGHTS.has(WHITE_KS), false);
        assert_eq!(NO_RIGHTS.has(BLACK_KS), false);
    }

    #[test]
    fn can_7() {
        assert_eq!(ALL_RIGHTS.has(WHITE_QS), true);
        assert_eq!(ALL_RIGHTS.has(BLACK_QS), true);
        assert_eq!(ALL_RIGHTS.has(WHITE_KS), true);
        assert_eq!(ALL_RIGHTS.has(BLACK_KS), true);
    }

    #[test]
    fn side_can_1() {
        let rights = BLACK_KS;

        assert_eq!(rights.side_can(WHITE), false);
        assert_eq!(rights.side_can(BLACK), true);
    }

    #[test]
    fn side_can_2() {
        let mut rights = WHITE_KS;
        rights.add(QUEEN_SIDE, WHITE);

        assert_eq!(rights.side_can(WHITE), true);
        assert_eq!(rights.side_can(BLACK), false);

        assert_eq!(NO_RIGHTS.side_can(WHITE), false);
        assert_eq!(NO_RIGHTS.side_can(BLACK), false);

        assert_eq!(ALL_RIGHTS.side_can(WHITE), true);
        assert_eq!(ALL_RIGHTS.side_can(BLACK), true);
    }

    #[test]
    fn any() {
        assert_eq!(CastlingRights::from(KING_SIDE, WHITE).any(), true);
        assert_eq!(NO_RIGHTS.any(), false);
    }


    #[test]
    fn char_1() {
        assert_eq!(WHITE_QS.to_string(), "Q");
    }

    #[test]
    fn char_2() {
        assert_eq!(BLACK_QS.to_string(), "q");
    }

    #[test]
    fn char_3() {
        assert_eq!(WHITE_KS.to_string(), "K");
    }

    #[test]
    fn char_4() {
        assert_eq!(BLACK_KS.to_string(), "k");
    }
}
