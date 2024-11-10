use std::fmt;

use super::Symbol;

// 产生式规则
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Production {
    pub lhs: Symbol,
    pub rhs: Vec<Symbol>,
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.lhs,
            self.rhs
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
