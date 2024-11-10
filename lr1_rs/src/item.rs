use super::Grammar;
use super::Production;
use super::Symbol;
use std::collections::{BTreeSet, HashMap, HashSet};

// LR(1)项
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LR1Item {
    pub production: Production, // 产生式
    pub dot_position: usize,    // 点的位置
    pub lookahead: Symbol,      // 向前看符号
}

// 语法规则集合

// 实现LR(1)项集的闭包计算
impl LR1Item {
    pub fn closure(
        &self,
        grammar: &Grammar,
        first: &HashMap<Symbol, HashSet<Symbol>>,
    ) -> BTreeSet<LR1Item> {
        let mut closure = BTreeSet::new();
        closure.insert(self.clone());

        let mut changed = true;
        while changed {
            changed = false;
            let current_items: Vec<LR1Item> = closure.iter().cloned().collect();

            for item in current_items {
                if item.dot_position >= item.production.rhs.len() {
                    continue;
                }

                let next_symbol = &item.production.rhs[item.dot_position];
                if let Symbol::NonTerminal(_) = next_symbol {
                    let beta = &item.production.rhs[item.dot_position + 1..];
                    let mut first_beta = HashSet::new();
                    first_beta.insert(item.lookahead.clone());

                    for symbol in beta {
                        let first_symbol = &first[symbol];
                        first_beta = first_symbol
                            .iter()
                            .filter(|&x| *x != Symbol::Epsilon)
                            .cloned()
                            .collect();
                        if !first_symbol.contains(&Symbol::Epsilon) {
                            break;
                        }
                    }

                    for production in &grammar.productions {
                        if &production.lhs == next_symbol {
                            for lookahead in &first_beta {
                                let new_item = LR1Item {
                                    production: production.clone(),
                                    dot_position: 0,
                                    lookahead: lookahead.clone(),
                                };
                                if closure.insert(new_item) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        closure
    }
}
