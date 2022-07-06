use super::errors;
use super::Token;

pub fn tokenize(s: String) -> Result<Vec<Token>, String> {
    let mut chars = s.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '\r' {
            chars.next();
            continue;
        }
        let token = match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '^' => Token::Exp,
            '=' => Token::Equal,
            ';' => Token::SemiColon,
            ' ' | '\n' => continue,
            'a'..='z' | 'A'..='Z' => {
                let mut s = String::from(c);
                while let Some(c) = chars.peek() {
                    let c = match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => chars.next(),
                        _ => break,
                    };
                    s.push(c.unwrap());
                }

                Token::Identifier(s).to_keyword()
            }

            '0'..='9' | '.' => {
                let mut s = String::from(c);

                while let Some(c) = chars.peek() {
                    if c == &'\r' {
                        chars.next();
                        continue;
                    }
                    let c = match c {
                        '0'..='9' | '.' => chars.next(),
                        _ => break,
                    };
                    s.push(c.unwrap());
                }
                Token::Number(s.parse().unwrap())
            }

            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.peek() {
                    if c == &'\r' {
                        chars.next();
                        continue;
                    }
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '_' => {
                            s.push(chars.next().unwrap());
                        }
                        '"' => {
                            chars.next();
                            break;
                        }
                        '\\' => {
                            chars.next();
                            if let Some(c) = chars.next() {
                                match c {
                                    'n' => s.push('\n'),
                                    'r' => s.push('\r'),
                                    't' => s.push('\t'),
                                    '\\' => s.push('\\'),
                                    '"' => s.push('"'),
                                    _ => {
                                        return Err(errors::unknown_escape_sequence(c));
                                    }
                                }
                            }
                        }

                        _ => {
                            return Err(errors::unexpected_character(*c));
                        }
                    }
                }

                Token::String(s)
            }

            '#' => {
                if let Some('#') = chars.peek() {
                    chars.next();
                    let mut s = String::new();

                    while let Some(c) = chars.peek() {
                        if c == &'\r' {
                            chars.next();
                            continue;
                        }
                        let c = match c {
                            '\n' => break,
                            _ => chars.next(),
                        };
                        s.push(c.unwrap());
                    }
                    Token::Comment(s)
                } else {
                    return Err(errors::unexpected_character(c));
                }
            }

            '/' => {
                if let Some('/') = chars.peek() {
                    chars.next();
                    Token::FloorDiv
                } else {
                    Token::Div
                }
            }

            _ => return Err(errors::unexpected_character(c)),
        };

        tokens.push(token);
    }

    Ok(tokens)
}
