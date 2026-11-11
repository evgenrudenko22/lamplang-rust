use std::collections::HashMap;
use std::io;

/* #region Lexer */

#[derive(Clone, Copy, PartialEq)]
enum TokenType {
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Star, Slash, 
    Bang, Equal, Less, Greater, 
    BangEqual, EqualEqual,
    LessEqual, GreaterEqual,
    Number, String, Identifier,
    Eof,
}

impl TokenType {
    fn to_string(&self) -> String {
        match self {
            Self::And => "and",
            Self::Class => "class",
            Self::Else => "else",
            Self::False => "false",
            Self::For => "for",
            Self::Fun => "fun",
            Self::If => "if",
            Self::Nil => "nil",
            Self::Or => "or",
            Self::Print => "print",
            Self::Return => "return",
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Semicolon => ";",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Bang => "!",
            Self::Equal => "=",
            Self::Less => "<",
            Self::Greater => ">",
            Self::BangEqual => "!=",
            Self::EqualEqual => "==",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Identifier => "IDENTIFIER",
            Self::Eof => "EOF",
        }
        .to_owned()
    }
}

#[derive(Clone)]
enum LiteralValue {
    Null,
    Number(f32),
    String(String),
    Identifier(String),
}

impl LiteralValue {
    fn as_str(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            _ => String::new()
        }
    }

    fn as_num(&self) -> f32 {
        match self {
            Self::Number(n) => *n,
            _ => 0f32
        }
    }
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    value: LiteralValue,
}

macro_rules! token {
    ($token_type: expr, $lexeme: expr, $value: expr, $tokens: ident) => {
        $tokens.push(Token { token_type: $token_type, lexeme: $lexeme, value: $value });
    };
    ($token_type: expr, $lexeme: expr, $tokens: ident) => {
        token!($token_type, $lexeme, LiteralValue::Null, $tokens);
    };
}

fn get(bytes: &[u8], i: usize) -> char {
    if i >= bytes.len() {
        return ' '
    }
    bytes[i] as char
}

fn lexer(src: String) -> Vec<Token> {
    use TokenType as Type;
    let keywords: HashMap<&str, TokenType> = [
        ("and", Type::And), ("class", Type::Class), ("else", Type::Else), ("false", Type::False),
        ("for", Type::For), ("fun", Type::Fun), ("if", Type::If), ("nil", Type::Nil),
        ("or", Type::Or), ("print", Type::Print), ("return", Type::Return), ("super", Type::Super),
        ("this", Type::This), ("true", Type::True), ("var", Type::Var), ("while", Type::While),
    ].iter().copied().collect();
    let singles: HashMap<char, TokenType> = [
        ('(', Type::LeftParen), (')', Type::RightParen), ('{', Type::LeftBrace), ('}', Type::RightBrace),
        (',', Type::Comma), ('.', Type::Dot), ('-', Type::Minus), ('+', Type::Plus),
        (';', Type::Semicolon), ('*', Type::Star), ('/', Type::Slash),
        ('!', Type::Bang), ('=', Type::Equal), ('<', Type::Less),  ('>', Type::Greater),
    ].iter().copied().collect();

    let bytes = src.as_bytes();
    let mut i = 0usize;
    let mut line = 1;
    let mut tokens: Vec<Token> = vec![];
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\n' { line += 1; i += 1; continue; }
        if c == ' ' || c == '\r' || c == '\t' { i += 1; continue; }
        if c == '/' && i + 1 < bytes.len() {
            if bytes[i + 1] as char == '/' {
                loop {
                    //println!("{}", bytes[i] as char);
                    i += 1;
                    if bytes[i] as char == '\n' {
                        line += 1; break;
                    }
                }
            } else if bytes[i + 1] as char == '*' {
                let start_line = line;
                loop {
                    i += 1;
                    if i == bytes.len() {
                        eprintln!("[line {}] Error: Unterminated comment", start_line);
                        break;
                    }
                    if bytes[i] as char == '\n' {
                        line += 1;
                        continue;
                    }
                    if bytes[i] as char == '*' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
                        i += 2;
                        break;
                    }
                }
            }
        } else if "!=<>".contains(c) && i + 1 < bytes.len() && bytes[i + 1] as char == '=' {
            let lexeme = format!("{}{}", c, bytes[i + 1] as char);
            token!(match lexeme.as_str() {
                "!=" => Type::BangEqual,
                "==" => Type::EqualEqual,
                "<=" => Type::LessEqual,
                ">=" => Type::GreaterEqual,
                _ => unreachable!(),
            }, lexeme, tokens); 
            i += 2; continue;
        } else if let Some(t) = singles.get(&c) {
            token!(t.clone(), c.to_string(), tokens); i += 1; continue;
        } else if c == '"' {
            let start = i;
            let mut string = String::new();
            loop {
                i += 1;
                let c = bytes[i] as char;
                if i == bytes.len() {
                    eprintln!("[line {}] Error: Unterminated string", line);
                    break;
                }
                if c == '\\' {
                    string.push(match bytes[i + 1] as char {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '\\' => '\\',
                        '"' => '"',
                        c => c
                    });
                    i += 1;
                    continue;
                }
                if c == '\n' {
                    line += 1;
                }
                if c == '"' {
                    token!(Type::String, String::from_utf8(bytes[start..i + 1].to_vec()).unwrap(), LiteralValue::String(string), tokens);
                    break;
                }
                string.push(c);
            }
        } else if c.is_digit(10u32) {
            let mut num = String::new();
            num.push(c);
            let mut c = get(bytes, i + 1);
            while c.is_digit(10u32) {
                i += 1;
                num.push(c);
                c = get(bytes, i + 1);
            }
            if c == '.' {
                i += 1;
                num.push(c);
                c = get(bytes, i + 1);
                while c.is_digit(10u32) {
                    i += 1;
                    num.push(c);
                    c = get(bytes, i + 1);
                }
            }

            token!(Type::Number, num.clone(), LiteralValue::Number(num.parse::<f32>().unwrap()), tokens);
        } else if c.is_alphabetic() {
            let mut ident = String::new();
            ident.push(c);
            let mut c = get(bytes, i + 1);
            while c.is_alphabetic() || c == '_' {
                i += 1;
                ident.push(c);
                c = get(bytes, i + 1);
            }

            if let Some(token_type) = keywords.get(ident.as_str()) {
                token!(*token_type, ident, tokens);
            } else {
                token!(Type::Identifier, ident.clone(), LiteralValue::Identifier(ident), tokens);
            }
        } else {
            eprintln!("[line {}] Error: Unexpected character: {}", line, c);
        }
        i += 1;
    }
    token!(Type::Eof, String::new(), tokens);
    tokens
}

/* #endregion */

/* #region Parser */

#[derive(Clone, PartialEq)]
enum Value {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Value {
    fn as_num(&self) -> f32 {
        match self {
            Value::Number(v) => *v,
            _ => panic!()
        }
    }
    
    fn is_falsy(&self) -> bool {
        use Value::*;
        match self {
            Number(v) => *v == 0.0,
            String(v) => v.is_empty(),
            True => false,
            False => true,
            Nil => true
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
enum Expr {
    Lit(Value),
    Unary(TokenType, Box<Expr>),
    Binary(Box<Expr>, TokenType, Box<Expr>),
    Group(Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(v) => write!(f, "{}", v),
            Self::Unary(t, expr) => {
                write!(f, "({} {})", t.to_string(), *expr)
            }
            Self::Binary(expr1, t, expr2) => {
                write!(f, "({} {} {})", t.to_string(), *expr1, *expr2)
            }
            Self::Group(expr) => {
                write!(f, "(group {})", *expr)
            }
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn check(&self, t: TokenType) -> bool { self.tokens[self.current].token_type == t }
    fn advance(&mut self) -> Token { let t = self.tokens[self.current].clone(); self.current += 1; t }
    fn matches(&mut self, ts: &[TokenType]) -> bool {
        for t in ts {
            if self.check(*t) { self.advance(); return true; }
        }
        false
    }

    fn expression(&mut self) -> Expr { self.equality() }
    fn equality(&mut self) -> Expr {
        let mut left = self.comparision();
        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.tokens[self.current-1].token_type.clone();
            let right = self.comparision();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn comparision(&mut self) -> Expr {
        let mut left = self.term();
        while self.matches(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.tokens[self.current-1].token_type.clone();
            let right = self.term();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let op = self.tokens[self.current-1].token_type.clone();
            let right = self.factor();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn factor(&mut self) -> Expr {
        let mut left = self.unary();
        while self.matches(&[TokenType::Star, TokenType::Slash]) {
            let op = self.tokens[self.current-1].token_type.clone();
            let right = self.unary();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    fn unary(&mut self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.tokens[self.current-1].token_type.clone();
            return Expr::Unary(op, Box::new(self.unary()));
        }
        self.primary()
    }
    fn primary(&mut self) -> Expr {
        let t = self.advance();
        match t.token_type {
            TokenType::Number => Expr::Lit(Value::Number(t.value.as_num())),
            TokenType::String => Expr::Lit(Value::String(t.value.as_str())),
            TokenType::True => Expr::Lit(Value::True),
            TokenType::False => Expr::Lit(Value::False),
            TokenType::Nil => Expr::Lit(Value::Nil),
            TokenType::LeftParen => {
                let g = Expr::Group(Box::new(self.expression()));
                self.advance();
                g
            }
            _ => panic!("Expect expression")
        }
    }
}

/* #endregion */

/* #region Interpreter */

fn bool_value(b: bool) -> Value {
    if b { Value::True } else { Value::False }
}

fn evaluate(expr: Expr) -> Value {
    use Expr::*;
    match expr {
        Lit(v) => v,
        Unary(t, expr) => {
            let v = evaluate(*expr);
            match t {
                TokenType::Minus => Value::Number(-v.as_num()),
                TokenType::Bang => bool_value(v.is_falsy()), 
                _ => Value::Nil,
            }
        }
        Binary(expr1, t, expr2) => {
            let v1 = evaluate(*expr1);
            let v2 = evaluate(*expr2);
            match t {
                TokenType::Plus => {
                    if let Value::Number(n1) = v1 {
                        if let Value::Number(n2) = v2 {
                            return Value::Number(n1 + n2);
                        }
                    } else if let Value::String(s1) = v1 {
                        if let Value::String(s2) = v2 {
                            return Value::String(format!("{}{}", s1, s2));
                        }
                    }
                    panic!()
                },
                TokenType::Minus => Value::Number(v1.as_num() - v2.as_num()),
                TokenType::Star => Value::Number(v1.as_num() * v2.as_num()),
                TokenType::Slash => Value::Number(v1.as_num() / v2.as_num()),
                TokenType::EqualEqual => bool_value(v1 == v2),
                TokenType::BangEqual => Value::Number(v1 != v2),
                TokenType::Less => Value::Number(v1 < v2),
                TokenType::LessEqual => Value::Number(v1 <= v2),
                TokenType::Greater => Value::Number(v1 > v2),
                TokenType::GreaterEqual => Value::Number(v1 >= v2),
                _ => Value::Nil
            }
        }
        Group(expr) => evaluate(*expr)
    }
}

/* #endregion */

fn main() {
    for line in io::stdin().lines() {
        if let Ok(line) = line {
            let tokens = lexer(line);
            let mut parser = Parser::new(tokens);
            println!("{}", evaluate(parser.expression()));
        } else {
            break;
        }
    }

}
