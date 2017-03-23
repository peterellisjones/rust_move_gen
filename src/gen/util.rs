#[cfg(test)]
use mv_list::MoveVec;

#[cfg(test)]
pub fn assert_list_includes_moves(list: &MoveVec, moves: &[&'static str]) {
    for &m in moves.iter() {
        assert!(list.iter().map(|m| m.to_string()).any(|mv| mv == m));
    }
}

#[cfg(test)]
pub fn assert_list_excludes_moves(list: &MoveVec, moves: &[&'static str]) {
    for &m in moves.iter() {
        assert!(list.iter().map(|m| m.to_string()).all(|mv| mv != m));
    }
}
