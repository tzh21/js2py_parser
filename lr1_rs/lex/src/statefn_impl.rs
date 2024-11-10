//! The implementation of the state functions that the lexer uses to parse the input string.

use crate::lexer::Lexer;
use crate::statefn::StateFn;
use crate::token::Type;

impl Default for StateFn {
    fn default() -> StateFn {
        StateFn::from(lex_start)
    }
}

pub fn lex_start(lexer: &mut Lexer) -> Option<StateFn> {
    if lexer.eof() {
        lexer.send(Type::EOF, "".to_string());
        return None;
    }

    let c = lexer.peek();

    // trivial cases:
    if let Some(next_f) = match c.unwrap() {
        ';' => Some(Type::SemiColon),
        '(' => Some(Type::LeftParen),
        ')' => Some(Type::RightParen),
        '{' => Some(Type::LeftBrace),
        '}' => Some(Type::RightBrace),
        '+' => Some(Type::Plus),
        '*' => Some(Type::Multiply),
        '/' => Some(Type::Divide),
        '>' => Some(Type::Greater),
        '<' => Some(Type::Less),
        _ => None,
    } {
        lexer.step();
        lexer.emit(next_f);
        return Some(StateFn::from(lex_start));
    }

    match c.unwrap() {
        'a'..='z' | 'A'..='Z' => Some(StateFn::from(lex_alpha)),
        '0'..='9' => Some(StateFn::from(lex_number)),
        '"' => Some(StateFn::from(lex_string_literal)),
        '-' => Some(StateFn::from(lex_minus)), // '-' or number
        '=' => Some(StateFn::from(lex_eq_op)), // '=' or '=='

        // all blank characters
        ' ' | '\t' | '\n' | '\r' => {
            lexer.step();
            lexer.ignore();
            Some(StateFn::from(lex_start))
        }

        _ => lex_error(
            lexer,
            format!("unexpected character: '{}'", c.unwrap()).as_str(),
        ),
    }
}

fn lex_eq_op(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run("=");
    let s = lexer.current();
    match s.as_str() {
        "=" => lexer.emit(Type::Assign),
        "==" => lexer.emit(Type::Equal),
        _ => unreachable!(),
    }
    Some(StateFn::from(lex_start))
}

fn lex_minus(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    if let Some('0'..='9') = lexer.peek() {
        Some(StateFn::from(lex_number))
    } else {
        lexer.emit(Type::Minus);
        Some(StateFn::from(lex_start))
    }
}

fn lex_number(lexer: &mut Lexer) -> Option<StateFn> {
    if let Some('0'..='9') = lexer.peek() {
        lexer.step();
        Some(StateFn::from(lex_number))
    } else {
        lexer.emit(Type::Number);
        Some(StateFn::from(lex_start))
    }
}

static SPECIAL_CHARS: &str = " !#$%&'()*+,-./:;<=>?@[\\]^_{|}~";
static DIGITS_AND_ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn lex_string_literal(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    while let Some(c) = lexer.peek() {
        if c == '"' {
            lexer.step();
            lexer.emit(Type::StringLiteral);
            return Some(StateFn::from(lex_start));
        }
        if !SPECIAL_CHARS.contains(c) && !DIGITS_AND_ALPHA.contains(c) {
            return lex_error(
                lexer,
                format!("unexpected character in string literal: '{}'", c).as_str(),
            );
        }
        lexer.step();
    }
    lex_error(lexer, "unterminated string literal")
}

fn lex_alpha(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run(DIGITS_AND_ALPHA);
    let s = lexer.current();
    match s.as_str() {
        "var" => lexer.emit(Type::Var),
        "input" => lexer.emit(Type::Input),
        "if" => lexer.emit(Type::If),
        "print" => lexer.emit(Type::Print),
        "while" => lexer.emit(Type::While),
        _ => lexer.emit(Type::Identifier),
    }
    Some(StateFn::from(lex_start))
}

fn lex_error(lexer: &mut Lexer, msg: &str) -> Option<StateFn> {
    lexer.send(Type::LexerError, msg.to_string());
    None
}
