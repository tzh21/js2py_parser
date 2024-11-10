use std::{fs::File, io::Write, path::Path};

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

fn process(parser: &Parser, symbols: Vec<Symbol>, output_path: String) {
    let result = parser.parse(symbols);
    match result {
        Ok(tree) => {
            println!("Accepted");
            // println!("Parse tree: {:#?}", tree);

            // 序列化解析树并保存为 JSON 文件
            if let Ok(json) = serde_json::to_string_pretty(&tree) {
                let mut file = File::create(output_path.clone()).expect("Unable to create file");
                file.write_all(json.as_bytes()).expect("Unable to write data");
                println!("Parse tree saved to {}", output_path.clone())
            } else {
                eprintln!("Error serializing parse tree to JSON");
            }
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


    // 将 filename 转换为 Path 并获取不带路径的文件名
    let without_dir = Path::new(filename)
        .file_name() // 提取文件名
        .and_then(|name| name.to_str()) // 转换为 &str
        .unwrap_or(""); // 如果为空，返回空字符串
    process(&parser, symbols, format!("output/{}.json", without_dir));
}

#[cfg(test)]
mod test {
    // use lr1_rs::*;
    // use super::*;

    // #[test]
    // fn test1() {
    //     let symbols: Vec<_> = ["import", "str", "semicolon", "eof"]
    //         .iter()
    //         .map(|s| Term!(s))
    //         .collect();
    //     let parser = Parser::new(js_grammar());
    //     process(&parser, symbols);
    // }
    // #[test]
    // fn test2() {
    //     let symbols = symbolize("import 'yl';");
    //     let parser = Parser::new(js_grammar());
    //     process(&parser, symbols);
    // }
}
