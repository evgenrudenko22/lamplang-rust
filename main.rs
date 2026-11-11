use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn get(bytes: &[u8], i: usize) -> char {
    if i >= bytes.len() {
        return ' '
    }
    bytes[i] as char
}

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    let keywords: HashSet<&str> = ["and","class","else","false","for","fun","if","nil",
        "or","print","return","super","this","true","var","while"].iter().copied().collect();
    let singles: HashMap<char, &str> = [
        ('(', "LEFT_PAREN"), (')', "RIGHT_PAREN"), ('{', "LEFT_BRACE"), ('}', "RIGHT_BRACE"),
        (',', "COMMA"), ('.', "DOT"), ('-', "MINUS"), ('+', "PLUS"),
        (';', "SEMICOLON"), ('*', "STAR"), ('/', "SLASH"),
        ('!', "BANG"), ('=', "EQUAL"), ('<', "LESS"),  ('>', "GREATER"),
    ].iter().copied().collect();

    let bytes = src.as_bytes();
    let mut i = 0usize;
    let mut line = 1;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\n' { line += 1; i += 1; continue; }
        if c == ' ' || c == '\r' || c == '\t' { i += 1; continue; }
        if c == '/' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
            loop {
                i += 1;
                if bytes[i] as char == '\n' {
                    line += 1; i += 1; continue;
                }
            }
        }
        if "!=<>".contains(c) && i + 1 < bytes.len() && bytes[i + 1] as char == '=' {
            let lexeme = format!("{}{}", c, bytes[i + 1] as char);
            println!("{} {} null", match lexeme.as_str() {
                "!=" => "BANG_EQUAL",
                "==" => "EQUAL_EQUAL",
                "<=" => "LESS_EQUAL",
                ">=" => "GREATER_EQUAL",
                _ => "",
            }, lexeme); 
            i += 2; continue;
        } else if let Some(t) = singles.get(&c) {
            println!("{} {} null", t, c); i += 1; continue;
        } else if c == '"' {
            let mut string = String::new();
            loop {
                i += 1;
                let c = bytes[i] as char;
                if i == bytes.len() {
                    println!("unterminated string");
                    break;
                }
                if c == '"' {
                    println!("STRING {} {}", format!("\"{}\"", string), string);
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
                num.push(c);
                c = get(bytes, i + 1);
                while c.is_digit(10u32) {
                    i += 1;
                    num.push(c);
                    c = get(bytes, i + 1);
                }
            }

            println!("NUMBER {} {:?}", num, num.parse::<f32>().unwrap());
        } else if c.is_alphabetic() {
            let mut ident = String::new();
            ident.push(c);
            i += 1;
            let mut c = get(bytes, i);
            while c.is_alphabetic() || c == '_' {
                ident.push(c);
                i += 1;
                c = get(bytes, i);
            }

            if keywords.contains(&*ident) {
                println!("{} {} null", ident.to_uppercase(), ident);
            } else {
                println!("IDENTIFIER {} null", ident);
            }
        }
        i += 1;
    }
    println!("EOF  null");
    let _ = line;
}
