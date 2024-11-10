use js_lexer::lexer::*;
use lr1_rs::*;

fn js_grammar() -> Grammar {
    grammar!(
        "program",
        //
        prod!("program" => "sourceElements", "eof"),
        prod!("program" => "eof"),
        //
        prod!("sourceElements" => "sourceElement", "sourceElements"),
        prod!("sourceElements" => "sourceElement"),
        prod!("sourceElement" => "statement"),
        prod!("statementList" => "statement"),
        prod!("statementList" => "statement", "statementList"),
        //
        prod!("statement" => "block"),
        prod!("statement" => "variableStatement"),
        prod!("statement" => "importStatement"),
        prod!("statement" => "exportStatement"),
        prod!("statement" => "emptyStatement_"),
        prod!("statement" => "classDeclaration"),
        prod!("statement" => "functionDeclaration"),
        prod!("statement" => "expressionStatement"),
        prod!("statement" => "ifStatement"),
        prod!("statement" => "iterationStatement"),
        prod!("statement" => "continueStatement"),
        prod!("statement" => "breakStatement"),
        prod!("statement" => "returnStatement"),
        prod!("statement" => "yieldStatement"),
        prod!("statement" => "withStatement"),
        prod!("statement" => "labelledStatement"),
        prod!("statement" => "switchStatement"),
        prod!("statement" => "throwStatement"),
        prod!("statement" => "tryStatement"),
        prod!("statement" => "debuggerStatement"),
        prod!("importStatement" => "import", "importFromBlock"),
        prod!("importFromBlock" => "str", "eos"),
        prod!("eos" => "eof"),
        prod!("eos" => "semicolon")
    )
}

fn process(parser: &Parser, symbols: Vec<Symbol>) {
    let result = parser.parse(symbols);
    match result {
        Ok(tree) => {
            println!("Parse tree: {:#?}", tree);
        }
        Err(e) => {
            eprintln!("Error parsing: {}", e);
        }
    }
}

fn symbolize(source: &str) -> Vec<Symbol> {
    token_stream(source)
        .iter()
        .map(|token| Term!(format!("{:?}", token.typ).to_lowercase()))
        .filter(|s| s != &Term!("lineterminator"))
        .collect()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = std::fs::read_to_string(filename);
    let source = match source {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    let symbols = symbolize(&source);
    let parser = Parser::new(js_grammar());
    process(&parser, symbols);
}

#[cfg(test)]
mod test {
    use lr1_rs::*;
    use super::*;

    #[test]
    fn test1() {
        let symbols: Vec<_> = ["import", "str", "semicolon", "eof"]
            .iter()
            .map(|s| Term!(s))
            .collect();
        let parser = Parser::new(js_grammar());
        process(&parser, symbols);
    }
    // #[test]
    // fn test2() {
    //     let symbols = symbolize("import 'yl';");
    //     let parser = Parser::new(js_grammar());
    //     process(&parser, symbols);
    // }
}
