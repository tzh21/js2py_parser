mod grammar;
mod item;
mod parser;
mod prod;
mod symbol;
pub use grammar::*;
pub use item::*;
pub use parser::*;
pub use prod::*;
pub use symbol::*;

pub static FAKE_START: &str = "S'";

pub struct ProdShortCut {
    pub lhs: String,
    pub rhs: Vec<String>,
}

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! NonTerm {
        ($name:expr) => {
            Symbol::NonTerminal($name.to_string())
        };
    }
    #[macro_export]
    macro_rules! Term {
        ($name:expr) => {
            Symbol::Terminal($name.to_string())
        };
    }
    #[macro_export]
    macro_rules! prod {
        ($lhs:expr => $($rhs:expr),*) => {
            ProdShortCut { lhs: $lhs.to_string(), rhs: vec![$($rhs.to_string()),*] }
        };
    }

    #[macro_export]
    macro_rules! grammar {
        ($start:expr, $($prodshortcut:expr),*) => {
            {
                let mut grammar = Grammar::new(Symbol::NonTerminal($start.to_string()));

                let prods = vec![$($prodshortcut),*];
                let mut non_terminals = std::collections::HashSet::new();
                for p in &prods {
                    non_terminals.insert(p.lhs.clone());
                }
                let mut terminals = std::collections::HashSet::new();
                for p in &prods {
                    for s in &p.rhs {
                        if !non_terminals.contains(s) {
                            terminals.insert(s.clone());
                        }
                    }
                }

                for p in prods {
                    grammar.add_production(NonTerm!(p.lhs), p.rhs.iter().map(|x| {
                        if non_terminals.contains(x) {
                            NonTerm!(x)
                        } else {
                            Term!(x)
                        }
                    }).collect());
                }

                grammar
            }
        };
    }
}
