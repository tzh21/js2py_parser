use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use serde::{Serialize, Deserialize};

// 语法分析器
pub struct Parser {
    pub grammar: Grammar,
    pub action_table: HashMap<(usize, Symbol), Action>,
    pub goto_table: HashMap<(usize, Symbol), usize>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(Production),
    Accept,
    Error,
}

#[derive(Debug, Clone)]
struct State {
    pub items: BTreeSet<LR1Item>,
    pub index: usize,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ASTNode {
    Terminal(String),
    NonTerminal(String, Vec<ASTNode>),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Shift(s) => write!(f, "Shift({})", s),
            Action::Reduce(p) => write!(f, "Reduce({})", p),
            Action::Accept => write!(f, "Accept"),
            Action::Error => write!(f, "Error"),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Terminal(s) => write!(f, "\x1b[0;32m{}\x1b[0m", s),
            ASTNode::NonTerminal(s, children) => {
                write!(f, "\x1b[0;34m{}\x1b[0m(", s)?;
                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", child)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Parser {
    pub fn new(grammar: Grammar) -> Self {
        let mut parser = Parser {
            grammar,
            action_table: HashMap::new(),
            goto_table: HashMap::new(),
        };

        parser.build_parsing_table();
        parser
    }
}

impl Parser {
    pub fn parse(&self, tokens: Vec<Symbol>) -> Result<ASTNode, String> {
        let mut pos = 0;
        let mut stack = vec![0];

        // 用于构建AST的节点栈
        let mut node_stack: Vec<ASTNode> = Vec::new();

        loop {
            let state = *stack.last().unwrap();
            let symbol = if pos < tokens.len() {
                &tokens[pos]
            } else {
                &Symbol::EndMarker
            };

            match self.action_table.get(&(state, symbol.clone())) {
                Some(Action::Shift(next_state)) => {
                    stack.push(*next_state);
                    // 将终结符添加到节点栈
                    if let Symbol::Terminal(term) = symbol.clone() {
                        node_stack.push(ASTNode::Terminal(term));
                    }
                    pos += 1;
                }
                Some(Action::Reduce(production)) => {
                    // 执行规约操作
                    let len = production.rhs.len();
                    for _ in 0..len {
                        stack.pop();
                    }

                    let prev_state = *stack.last().unwrap();
                    if let Some(next_state) =
                        self.goto_table.get(&(prev_state, production.lhs.clone()))
                    {
                        stack.push(*next_state);

                        // 构建AST节点
                        let mut children = Vec::new();
                        for _ in 0..len {
                            if let Some(node) = node_stack.pop() {
                                children.push(node);
                            }
                        }
                        children.reverse();

                        if let Symbol::NonTerminal(name) = &production.lhs {
                            node_stack.push(ASTNode::NonTerminal(name.clone(), children));
                        }
                    } else {
                        return Err("Invalid state transition".to_string());
                    }
                }
                Some(Action::Accept) => {
                    // 返回最终的AST根节点
                    return Ok(node_stack
                        .pop()
                        .unwrap_or(ASTNode::Terminal("empty".to_string())));
                }
                _ => {
                    return Err(format!(
                        "Syntax error at position {}, state: {:?}, symbol: {:?}",
                        pos, state, symbol
                    ));
                }
            }
        }
    }
    // 构建LR(1)项集族
    fn build_canonical_collection(&mut self) -> Vec<State> {
        let first = self.grammar.compute_first();
        let mut states = Vec::new();
        let mut state_map: HashMap<BTreeSet<LR1Item>, usize> = HashMap::new();

        // 创建初始项集
        let initial_production = Production {
            lhs: Symbol::NonTerminal(FAKE_START.to_string()),
            rhs: vec![self.grammar.start_symbol.clone()],
        };

        let initial_item = LR1Item {
            production: initial_production,
            dot_position: 0,
            lookahead: Symbol::EndMarker,
        };

        let initial_closure = initial_item.closure(&self.grammar, &first);
        state_map.insert(initial_closure.clone(), 0);
        states.push(State {
            items: initial_closure,
            index: 0,
        });

        let mut i = 0;
        while i < states.len() {
            let state = states[i].clone();

            // 获取所有可能的下一个符号
            let mut next_symbols = HashSet::new();
            for item in &state.items {
                if item.dot_position < item.production.rhs.len() {
                    next_symbols.insert(item.production.rhs[item.dot_position].clone());
                }
            }

            // 对每个符号构建goto集合
            for symbol in next_symbols {
                let mut goto_set = BTreeSet::new();

                // 构建goto项集
                for item in &state.items {
                    if item.dot_position < item.production.rhs.len()
                        && item.production.rhs[item.dot_position] == symbol
                    {
                        let mut new_item = item.clone();
                        new_item.dot_position += 1;
                        let closure = new_item.closure(&self.grammar, &first);
                        goto_set.extend(closure);
                    }
                }

                if !goto_set.is_empty() {
                    // 检查是否已存在相同的状态
                    if let Some(&existing_index) = state_map.get(&goto_set) {
                        self.goto_table.insert((i, symbol.clone()), existing_index);
                    } else {
                        let new_index = states.len();
                        state_map.insert(goto_set.clone(), new_index);
                        states.push(State {
                            items: goto_set,
                            index: new_index,
                        });
                        self.goto_table.insert((i, symbol.clone()), new_index);
                    }
                }
            }

            i += 1;
        }

        states
    }

    // 构建Action表和Goto表
    fn build_parsing_table(&mut self) {
        let states = self.build_canonical_collection();

        // 对每个状态构建Action表
        for state in &states {
            for item in &state.items {
                if item.dot_position < item.production.rhs.len() {
                    // 移入动作
                    let symbol = &item.production.rhs[item.dot_position];
                    if let Symbol::Terminal(_) = symbol {
                        if let Some(&next_state) =
                            self.goto_table.get(&(state.index, symbol.clone()))
                        {
                            self.action_table
                                .insert((state.index, symbol.clone()), Action::Shift(next_state));
                        }
                    }
                } else {
                    // 规约动作
                    if item.production.lhs == Symbol::NonTerminal(FAKE_START.to_string())
                        && item.lookahead == Symbol::EndMarker
                    {
                        self.action_table
                            .insert((state.index, Symbol::EndMarker), Action::Accept);
                    } else {
                        self.action_table.insert(
                            (state.index, item.lookahead.clone()),
                            Action::Reduce(item.production.clone()),
                        );
                    }
                }
            }
        }
    }
}
