use bb::*;
use square::*;
use unindent;

#[cfg(test)]
use test;

#[cfg(test)]
pub fn test_bishop_attacks<F: Fn(BB, BB) -> BB>(gen: F) {
    let bishop_pos = A1;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|.......#|8
        7|......#.|7
        6|.....#..|6
        5|....#...|5
        4|...#....|4
        3|..#.....|3
        2|.#......|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(BB::new(bishop_pos), occupied).to_string(), expected);

    let bishop_pos = H8;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|........|8
        7|......#.|7
        6|.....#..|6
        5|....#...|5
        4|...#....|4
        3|..#.....|3
        2|.#......|2
        1|#.......|1
          ABCDEFGH
        ");
    assert_eq!(gen(BB::new(bishop_pos), occupied).to_string(), expected);

    let bishop_pos = H1;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|#.......|8
        7|.#......|7
        6|..#.....|6
        5|...#....|5
        4|....#...|4
        3|.....#..|3
        2|......#.|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(BB::new(bishop_pos), occupied).to_string(), expected);

    let bishop_pos = G6;
    let occupied = BB_D3;

    let expected = unindent::unindent("
          ABCDEFGH
        8|....#...|8
        7|.....#.#|7
        6|........|6
        5|.....#.#|5
        4|....#...|4
        3|...#....|3
        2|........|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(BB::new(bishop_pos), occupied).to_string(), expected);
}

#[cfg(test)]
pub fn test_bishop_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let bishop_pos = A1;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|.......#|8
        7|......#.|7
        6|.....#..|6
        5|....#...|5
        4|...#....|4
        3|..#.....|3
        2|.#......|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(bishop_pos, occupied).to_string(), expected);

    let bishop_pos = H8;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|........|8
        7|......#.|7
        6|.....#..|6
        5|....#...|5
        4|...#....|4
        3|..#.....|3
        2|.#......|2
        1|#.......|1
          ABCDEFGH
        ");
    assert_eq!(gen(bishop_pos, occupied).to_string(), expected);

    let bishop_pos = H1;
    let occupied = EMPTY;

    let expected = unindent::unindent("
          ABCDEFGH
        8|#.......|8
        7|.#......|7
        6|..#.....|6
        5|...#....|5
        4|....#...|4
        3|.....#..|3
        2|......#.|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(bishop_pos, occupied).to_string(), expected);

    let bishop_pos = G6;
    let occupied = BB_D3;

    let expected = unindent::unindent("
          ABCDEFGH
        8|....#...|8
        7|.....#.#|7
        6|........|6
        5|.....#.#|5
        4|....#...|4
        3|...#....|3
        2|........|2
        1|........|1
          ABCDEFGH
        ");
    assert_eq!(gen(bishop_pos, occupied).to_string(), expected);
}

#[cfg(test)]
pub fn test_rook_attacks<F: Fn(BB, BB) -> BB>(gen: F) {
    let rook_pos = G6;
    let occupied = BB_C6;

    let expected = unindent::unindent("
          ABCDEFGH
        8|......#.|8
        7|......#.|7
        6|..####.#|6
        5|......#.|5
        4|......#.|4
        3|......#.|3
        2|......#.|2
        1|......#.|1
          ABCDEFGH
        ");
    assert_eq!(gen(BB::new(rook_pos), occupied).to_string(), expected);
}

#[cfg(test)]
pub fn test_rook_attacks_from_sq<F: Fn(Square, BB) -> BB>(gen: F) {
    let rook_pos = G6;
    let occupied = BB_C6;

    let expected = unindent::unindent("
          ABCDEFGH
        8|......#.|8
        7|......#.|7
        6|..####.#|6
        5|......#.|5
        4|......#.|4
        3|......#.|3
        2|......#.|2
        1|......#.|1
          ABCDEFGH
        ");
    assert_eq!(gen(rook_pos, occupied).to_string(), expected);
}

#[cfg(test)]
pub fn bench_attacks_from_squares<F: Fn(BB, BB) -> BB>(b: &mut test::Bencher, gen: F) {
    let cases = test_cases_from_squares(100);
    b.iter(|| -> BB {
        let mut ret = EMPTY;

        for &(from, occupied) in cases.iter() {
            ret = ret ^ gen(from, occupied);
        }

        ret
    });
}

#[cfg(test)]
fn test_cases_from_squares(size: isize) -> Vec<(BB, BB)> {
    let mut ret = Vec::new();
    for _ in 0..size {
        let sq1 = Square::random();
        let sq2 = Square::random();
        let from = BB::new(sq1) | BB::new(sq2);
        let occupied = BB::random(0.3) | from;
        ret.push((from, occupied));
    }
    ret
}
