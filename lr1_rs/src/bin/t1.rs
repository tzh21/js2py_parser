use lr1_rs::*;
use std::collections::HashSet;

fn build_t1_grammer() -> Grammar {
    grammar!(
        "E",
        prod!("E" => "(", "L", ",", "E", ")"),
        prod!("E" => "F"),
        prod!("L" => "L", ",", "E"),
        prod!("L" => "E"),
        prod!("F" => "(", "F", ")"),
        prod!("F" => "d")
    )
}

fn print_goto_table(parser: &Parser) {
    let (max_state, max_symbol) =
        parser
            .goto_table
            .keys()
            .fold((0, 0), |(max_state, max_symbol), (st, sy)| {
                (max_state.max(*st), max_symbol.max(sy.to_string().len()))
            });

    for state in 0..=max_state {
        for symbol in &parser.grammar.non_terminals {
            if let Some(next_state) = parser.goto_table.get(&(state, symbol.clone())) {
                println!(
                    "GOTO[{}, {:width$}] = {}",
                    state,
                    symbol,
                    next_state,
                    width = max_symbol
                );
            }
        }
    }
}

fn print_action_table(parser: &Parser) {
    let (max_state, max_symbol) =
        parser
            .action_table
            .keys()
            .fold((0, 0), |(max_state, max_symbol), (st, sy)| {
                (max_state.max(*st), max_symbol.max(sy.to_string().len()))
            });

    for state in 0..=max_state {
        for symbol in parser
            .grammar
            .terminals
            .union(&HashSet::from([Symbol::EndMarker]))
        {
            if let Some(action) = parser.action_table.get(&(state, symbol.clone())) {
                println!(
                    "ACTION[{}, {:width$}] = {}",
                    state,
                    symbol,
                    action,
                    width = max_symbol
                );
            }
        }
    }
}

fn main() {
    let grammar = build_t1_grammer();
    let parser = Parser::new(grammar);
    print_action_table(&parser);
    print_goto_table(&parser);
}

#[test]
fn test_simple_grammer() {
    let grammar = build_t1_grammer();
    let parser = Parser::new(grammar);

    let input = ["(", "d", ")"].iter().map(|&c| Term!(c)).collect();

    match parser.parse(input) {
        Ok(ast) => {
            println!("{}", ast);
        }
        Err(e) => panic!("Parsing error: {}", e),
    }
}
