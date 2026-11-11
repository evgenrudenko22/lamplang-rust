use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    let keywords: HashSet<&str> = ["and","class","else","false","for","fun","if","nil",
        "or","print","return","super","this","true","var","while"].iter().copied().collect();
    let singles: HashMap<char, &str> = [
        ('(', "LEFT_PAREN"), (')', "RIGHT_PAREN"), ('{', "LEFT_BRACE"), ('}', "RIGHT_BRACE"),
        (',', "COMMA"), ('.', "DOT"), ('-', "MINUS"), ('+', "PLUS"),
        (';', "SEMICOLON"), ('*', "STAR"),
    ].iter().copied().collect();
    let _ = (keywords, &singles);

    let bytes = src.as_bytes();
    let mut i = 0usize;
    let mut line = 1;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\n' { line += 1; i += 1; continue; }
        if c == ' ' || c == '\r' || c == '\t' { i += 1; continue; }
        if let Some(t) = singles.get(&c) {
            println!("{} {} null", t, c); i += 1; continue;
        }
        // TODO: two-char ops (!=, ==, <=, >=) and BANG/EQUAL/LESS/GREATER variants.
        // TODO: '/' may be SLASH or start a // comment.
        // TODO: '"' STRING — track newlines, error if unterminated.
        // TODO: digit -> NUMBER (with optional fraction).
        // TODO: alpha/_ -> IDENTIFIER, lookup in keywords.
        i += 1;
    }
    println!("EOF  null");
    let _ = line;
}
