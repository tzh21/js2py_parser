use crate::lexer::Lexer;

#[derive(Clone)]
/// StateFn is a wrapper around a function that returns the next state function to run
/// or None.
pub struct StateFn {
    f: fn(&mut Lexer) -> Option<StateFn>,
}

impl StateFn {
    pub fn from(f: fn(&mut Lexer) -> Option<StateFn>) -> StateFn {
        StateFn { f }
    }
    pub fn call(&self, lexer: &mut Lexer) -> Option<StateFn> {
        (self.f)(lexer)
    }
}
