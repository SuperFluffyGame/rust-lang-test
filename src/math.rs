use regex::Regex;

#[derive(Debug)]
pub struct Node {
    pub value: Token,
    pub children: Vec<Node>,
}
impl Node {
    pub fn new(value: Token) -> Node {
        Node {
            value,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}
impl Token {
    fn get_char(&self) -> char {
        match self {
            Token::Number(n) => n.to_string().chars().next().unwrap(),
            Token::Operator(op) => *op,
            Token::LeftParen => '(',
            Token::RightParen => ')',
        }
    }
}

fn is_digit(c: &char) -> bool {
    let regex = Regex::new(r"[\d\.]").unwrap();

    regex.is_match(&c.to_string())
}
fn is_operator(c: &char) -> bool {
    let regex = Regex::new(r"[+-/*^]").unwrap();

    regex.is_match(&c.to_string())
}
fn is_left_paren(c: &char) -> bool {
    c == &'('
}
fn is_right_paren(c: &char) -> bool {
    c == &')'
}
fn is_whitespace(c: &char) -> bool {
    let regex = Regex::new(r"\s").unwrap();

    regex.is_match(&c.to_string())
}

pub fn tokenize(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.next() {
        if is_operator(&c) {
            tokens.push(Token::Operator(c));
        } else if is_digit(&c) {
            let mut number = c.to_string();
            while let Some(c2) = chars.peek() {
                if is_digit(&c2) {
                    number.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(number.parse::<f64>().unwrap()));
        } else if is_left_paren(&c) {
            tokens.push(Token::LeftParen);
        } else if is_right_paren(&c) {
            tokens.push(Token::RightParen);
        } else if !is_whitespace(&c) {
            return Err(format!("Unknown Character: '{}'.\n", c).to_string());
        }
    }

    Ok(tokens)
}

fn get_operator_precedence(op: &char) -> i32 {
    match op {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn compare_operator_precedence(op1: &char, op2: &char) -> i32 {
    let o1p = get_operator_precedence(op1);
    let o2p = get_operator_precedence(op2);

    if o1p > o2p {
        1
    } else if o1p < o2p {
        -1
    } else {
        0
    }
}

fn is_operator_right_associative(op: &char) -> bool {
    match op {
        '^' => true,
        _ => false,
    }
}

fn eval_operator(op: &char, a: f64, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => Ok(a / b),
        '^' => Ok(a.powf(b)),
        _ => Err(format!("Unknown Operator: {}.\n", op).to_string()),
    }
}

fn add_node(stack: &mut Vec<Node>, op: Token) {
    match op {
        Token::Operator(_) => {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let mut node = Node::new(op);
            node.children.push(a);
            node.children.push(b);
            stack.push(node);
        }
        _ => {
            println!("Unknown Operator: {:?}", op);
        }
    }
}

pub fn parse_ast(tokens: &Vec<Token>) -> Node {
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut operand_stack: Vec<Node> = Vec::new();

    'main: for i in 0..tokens.len() {
        let token = &tokens[i];
        let mut popped: Token;
        match token {
            Token::LeftParen => {
                operator_stack.push(Token::LeftParen);
            }
            Token::RightParen => {
                while !operator_stack.is_empty() {
                    popped = operator_stack.pop().unwrap();
                    match popped {
                        Token::LeftParen => {
                            continue 'main;
                        }
                        _ => {
                            add_node(&mut operand_stack, popped);
                        }
                    }
                }
            }

            Token::Operator(o1) => {
                while !operator_stack.is_empty() {
                    let o2 = &operator_stack.last().unwrap().get_char();
                    if (!is_operator_right_associative(o1)
                        && compare_operator_precedence(o1, o2) == 0)
                        || compare_operator_precedence(o1, o2) < 0
                    {
                        print!("loop");
                        operator_stack.pop().unwrap();
                        add_node(&mut operand_stack, Token::Operator(*o2));
                    } else {
                        break;
                    };
                }
                operator_stack.push(Token::Operator(*o1));
            }
            Token::Number(_) => {
                operand_stack.push(Node::new(*token));
            }
        }
    }

    while !operator_stack.is_empty() {
        add_node(&mut operand_stack, operator_stack.pop().unwrap());
    }

    return operand_stack.pop().unwrap();
}

pub fn eval_ast(a: &Node) -> Result<f64, String> {
    match a.value {
        Token::Number(num) => Ok(num),
        Token::Operator(op) => {
            let b = eval_ast(&a.children[1])?;
            let a = eval_ast(&a.children[0])?;
            eval_operator(&op, a, b)
        }
        _ => Err("Unknown AST Node".to_string()),
    }
}
