pub fn unexpected_character(in_c: char) -> String {
    let c = match in_c {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        _ => in_c.to_string(),
    };

    format!("Unexpected character: '{}'", c)
}

pub fn unknown_escape_sequence(in_c: char) -> String {
    format!("Unknown escape sequence: '\\{}'", in_c)
}
