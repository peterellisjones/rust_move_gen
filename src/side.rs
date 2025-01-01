use std::fmt;

type SideInternal = usize;

/// Represents a side to move
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Side(pub SideInternal);

const CHARS: [char; 2] = ['w', 'b'];
const NAMES: [&str; 2] = ["white", "black"];

impl Side {
    pub fn to_char(self) -> char {
        CHARS[self.to_usize()]
    }

    pub fn to_usize(self) -> usize {
        self.0
    }

    pub fn raw(self) -> SideInternal {
        self.0
    }

    /// Flip switches sides
    pub fn flip(self) -> Side {
        Side(self.0 ^ 1)
    }

    pub fn parse(c: char) -> Result<Side, String> {
        for (i, _c) in CHARS.iter().enumerate() {
            if *_c == c {
                return Ok(Side(i));
            }
        }
        Err(format!("Side not recognised: {}", c))
    }

    pub fn to_str(self) -> &'static str {
        NAMES[self.0]
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Debug for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

pub const WHITE: Side = Side(0);
pub const BLACK: Side = Side(1);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char() {
        assert_eq!(BLACK.to_char(), 'b');
        assert_eq!(WHITE.to_char(), 'w');
    }

    #[test]
    fn flip() {
        assert_eq!(BLACK.flip(), WHITE);
        assert_eq!(WHITE.flip(), BLACK);
    }
}
