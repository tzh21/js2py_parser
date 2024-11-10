use std::fmt;

// 文法符号
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
    EndMarker,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Terminal(s) => write!(f, "{}", s),
            Symbol::NonTerminal(s) => write!(f, "{}", s),
            Symbol::Epsilon => write!(f, "ε"),
            Symbol::EndMarker => write!(f, "#"),
        }
    }
}
