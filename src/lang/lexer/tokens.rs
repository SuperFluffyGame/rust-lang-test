#[derive(Debug, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Number(f64),
    Identifier(String),
    String(String),
    Comment(String),

    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Exp,

    Equal,
    SemiColon,

    KwLet,
}
impl Token {
    pub fn to_keyword(&self) -> Token {
        match self {
            Token::Identifier(s) if s == "let" => Token::KwLet,
            _ => self.clone(),
        }
    }
}
