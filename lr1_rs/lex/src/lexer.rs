use crate::statefn::StateFn;
use crate::token::{Token, Type};
use std::sync::mpsc;

/// Create a token stream from the input string
///
/// * `input`: the input string to tokenize
pub fn token_stream(input: &str) -> mpsc::Receiver<Token> {
    let (tx, rx) = mpsc::channel();
    let mut l = Lexer {
        input: input.to_string(),
        start: 0,
        pos: 0,
        initial_state: StateFn::default(),
        sender: tx,
    };
    std::thread::spawn(move || l.run());
    rx
}

/// Lexer context
pub struct Lexer {
    input: String,
    start: usize,
    pos: usize,
    initial_state: StateFn,
    sender: mpsc::Sender<Token>,
}

impl Lexer {
    pub fn run(&mut self) {
        let mut f = self.initial_state.clone();

        // Run the state functions until there are no more state functions to run
        // This pattern can decouple the lexer from the state functions and make it easier to
        // extend the lexer with new state functions.
        while let Some(next_f) = f.call(self) {
            f = next_f;
        }
    }

    // The following methods are used by the state functions to interact with the lexer context:

    pub fn current(&self) -> String {
        self.input[self.start..self.pos].to_string()
    }
    /// Emit a token with the current value
    pub fn emit(&mut self, typ: Type) {
        let val = self.input[self.start..self.pos].to_string();
        self.send(typ, val);
        self.start = self.pos;
    }
    /// Send a token (without updating the start position)
    pub fn send(&mut self, typ: Type, val: String) {
        let _ = self.sender.send(Token { typ, val });
    }
    pub fn ignore(&mut self) {
        self.start = self.pos;
    }
    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    pub fn step(&mut self) -> Option<char> {
        self.pos += self.peek()?.len_utf8();
        self.peek()
    }
    pub fn accept(&mut self, valid: &str) -> bool {
        if let Some(c) = self.peek() {
            if valid.contains(c) {
                self.step();
                return true;
            }
        }
        false
    }
    pub fn accept_run(&mut self, valid: &str) -> bool {
        let mut accepted = false;
        while self.accept(valid) {
            accepted = true;
        }
        accepted
    }
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
