use lr1_rs::*;
use toy_lang_lexer::lexer::token_stream;
use toy_lang_lexer::token::*;

fn build_t2_grammar() -> Grammar {
    grammar!(
        "PROGRAM",
        prod!("PROGRAM" => "STATEMENT", "PROGRAM"),
        prod!("PROGRAM" => "STATEMENT"),
        prod!("PROGRAM" => ),
        prod!("STATEMENT" => "INPUT_STMT"),
        prod!("STATEMENT" => "PRINT_STMT"),
        prod!("STATEMENT" => "DECLARATION_STMT"),
        prod!("STATEMENT" => "ASSIGNMENT_STMT"),
        prod!("STATEMENT" => "IF_STMT"),
        prod!("STATEMENT" => "WHILE_STMT"),
        prod!("INPUT_STMT" => "input", "identifier", ";"),
        prod!("PRINT_STMT" => "print", "identifier", ";"),
        prod!("PRINT_STMT" => "print", "stringliteral", ";"),
        prod!("DECLARATION_STMT" => "var", "identifier", ";"),
        prod!("ASSIGNMENT_STMT" => "identifier", "=", "EXPRESSION", ";"),
        prod!("IF_STMT" => "if", "(", "CONDITION", ")", "{", "PROGRAM", "}"),
        prod!("WHILE_STMT" => "while", "(", "CONDITION", ")", "{", "PROGRAM", "}"),
        prod!("EXPRESSION" => "TERM"),
        prod!("EXPRESSION" => "TERM", "+", "TERM"),
        prod!("EXPRESSION" => "TERM", "-", "TERM"),
        prod!("TERM" => "FACTOR"),
        prod!("TERM" => "FACTOR", "*", "FACTOR"),
        prod!("TERM" => "FACTOR", "/", "FACTOR"),
        prod!("FACTOR" => "identifier"),
        prod!("FACTOR" => "number"),
        prod!("FACTOR" => "(", "EXPRESSION", ")"),
        prod!("CONDITION" => "EXPRESSION", "==", "EXPRESSION"),
        prod!("CONDITION" => "EXPRESSION", ">", "EXPRESSION"),
        prod!("CONDITION" => "EXPRESSION", "<", "EXPRESSION")
    )
}

fn print_xml(
    ast: &ASTNode,
    indent: usize,
    show_program: bool,
    mut tokens: Vec<Token>,
) -> Vec<Token> {
    match ast {
        ASTNode::NonTerminal(name, children) => {
            // if current ASTNODE is 'PROGRAM' and its second child 'PROGRAM',
            // then we should unwrap the second child 'PROGRAM' and print its sub-children
            if !show_program && name == "PROGRAM" {
                for child in children {
                    tokens = print_xml(child, indent, false, tokens);
                }
            } else {
                println!("{}\x1b[0;34m<{}>\x1b[0m", " ".repeat(indent), name);
                for child in children {
                    let show_program_child = !show_program
                        && matches!(child, ASTNode::NonTerminal(name, _) if name == "PROGRAM");
                    tokens = print_xml(child, indent + 2, show_program_child, tokens);
                }
                println!("{}\x1b[0;34m</{}>\x1b[0m", " ".repeat(indent), name);
            }
        }

        ASTNode::Terminal(_value) => {
            let token = tokens.first().unwrap();
            let name = format!("{:?}", token.typ).to_lowercase();
            println!(
                "{}\x1b[0;32m<{}>\x1b[0m{}\x1b[0;32m</{}>\x1b[0m",
                " ".repeat(indent),
                name,
                token.val,
                name
            );
            tokens.remove(0);
        }
    }
    tokens
}

fn process(text: &str) {
    let grammar = build_t2_grammar();
    let parser = Parser::new(grammar);

    let tokens = token_stream(text).iter().collect::<Vec<_>>();
    let preprocessed_tokens = tokens
        .iter()
        .filter(|token| token.typ != Type::EOF)
        .map(|token| match token.typ {
            Type::Identifier | Type::Number | Type::StringLiteral => {
                Term!(format!("{:?}", token.typ).to_lowercase())
            }
            _ => Term!(token.val),
        })
        .collect();

    match parser.parse(preprocessed_tokens) {
        Err(e) => {
            panic!("{}", e);
        }
        Ok(ast) => {
            print_xml(&ast, 0, true, tokens);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ./{{project_name}} <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    if !std::path::Path::new(filename).exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(filename).unwrap();
    process(&input);
}

#[test]
fn test_toy_lang() {
    process(
        r#"
        var x;
        var y;
        x = 1;
        y = 2;
        print x;
    "#,
    );
}

#[test]
fn test_toy_lang_empty() {
    process("");
}
