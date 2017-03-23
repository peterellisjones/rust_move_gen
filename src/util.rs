use square::Square;

pub fn grid_to_string_with_props<F: Fn(Square) -> char>(char_at: F,
                                                        props: &[(&str, String)])
                                                        -> String {
    let mut string = "  ABCDEFGH\n".to_string();

    let row_chars = ['1', '2', '3', '4', '5', '6', '7', '8'];

    for row in (0..8).rev() {
        string += &format!("{}|", row_chars[row]);
        for col in 0..8 {
            string.push(char_at(Square::from(row, col)));
        }
        if props.len() > (7 - row) {
            string += &format!("|{} {}: {}\n",
                               row_chars[row],
                               props[(7 - row)].0,
                               props[(7 - row)].1);
        } else {
            string += &format!("|{}\n", row_chars[row]);
        }
    }

    string + &"  ABCDEFGH\n".to_string()
}

pub fn grid_to_string<F: Fn(Square) -> char>(char_at: F) -> String {
    grid_to_string_with_props(char_at, &[])
}
