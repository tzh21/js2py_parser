use super::Production;
use super::Symbol;
use std::collections::{HashMap, HashSet};

// 语法规则
#[derive(Debug)]
pub struct Grammar {
    pub productions: Vec<Production>,
    pub terminals: HashSet<Symbol>,
    pub non_terminals: HashSet<Symbol>,
    pub start_symbol: Symbol,
}

impl Grammar {
    pub fn new(start_symbol: Symbol) -> Self {
        Grammar {
            productions: Vec::new(),
            terminals: HashSet::new(),
            non_terminals: HashSet::new(),
            start_symbol,
        }
    }

    // 添加产生式
    pub fn add_production(&mut self, lhs: Symbol, rhs: Vec<Symbol>) {
        match &lhs {
            Symbol::NonTerminal(_) => {
                self.non_terminals.insert(lhs.clone());
            }
            _ => panic!("LHS must be a non-terminal"),
        }

        for symbol in &rhs {
            match symbol {
                Symbol::Terminal(_) => {
                    self.terminals.insert(symbol.clone());
                }
                Symbol::NonTerminal(_) => {
                    self.non_terminals.insert(symbol.clone());
                }
                _ => {}
            }
        }

        self.productions.push(Production { lhs, rhs });
    }

    // 计算First集合
    pub fn compute_first(&self) -> HashMap<Symbol, HashSet<Symbol>> {
        let mut first: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();

        // 初始化
        for terminal in &self.terminals {
            let mut set = HashSet::new();
            set.insert(terminal.clone());
            first.insert(terminal.clone(), set);
        }

        for non_terminal in &self.non_terminals {
            first.insert(non_terminal.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;

            for production in &self.productions {
                let lhs = &production.lhs;
                let mut first_lhs = first[lhs].clone();

                // 计算RHS的First集
                let mut can_derive_epsilon = true;
                for symbol in production.rhs.iter().filter(|&s| *s != Symbol::Epsilon) {
                    if !can_derive_epsilon {
                        break;
                    }

                    let first_symbol = first[symbol].clone();
                    for terminal in first_symbol {
                        if terminal != Symbol::Epsilon && first_lhs.insert(terminal) {
                            changed = true;
                        }
                    }

                    can_derive_epsilon = first[symbol].contains(&Symbol::Epsilon);
                }

                if can_derive_epsilon && first_lhs.insert(Symbol::Epsilon) {
                    changed = true;
                }

                first.insert(lhs.clone(), first_lhs);
            }
        }

        first
    }
}
