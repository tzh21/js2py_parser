use std::{fs::File, io::Write, path::Path};

use js_lexer::lexer::*;
use lr1_rs::*;

fn js_grammar() -> Grammar {
    grammar!(
        "program",
        // Program structure
        prod!("program" => "sourceElements", "eof"),
        prod!("program" => "eof"),
        prod!("sourceElements" => "sourceElement", "sourceElements"),
        prod!("sourceElements" => "sourceElement"),
        prod!("sourceElement" => "statement"),
        prod!("statementList" => "statement"),
        prod!("statementList" => "statement", "statementList"),

        // Basic statements
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

        // Import statements
        prod!("importStatement" => "import", "importFromBlock"),
        prod!("importFromBlock" => "str", "eos"),

        // Variable declarations
        prod!("variableStatement" => "var", "variableDeclarationList", "eos"),
        prod!("variableStatement" => "let", "variableDeclarationList", "eos"),
        prod!("variableStatement" => "const", "variableDeclarationList", "eos"),
        prod!("variableDeclarationList" => "variableDeclaration"),
        prod!("variableDeclarationList" => "variableDeclaration", "comma", "variableDeclarationList"),
        prod!("variableDeclaration" => "identifier"),
        prod!("variableDeclaration" => "identifier", "assign", "singleExpression"),

        // Function declarations
        prod!("functionDeclaration" => "function", "identifier", "openParen", "closeParen", "block"),
        prod!("functionDeclaration" => "function", "identifier", "openParen", "formalParameterList", "closeParen", "block"),
        prod!("formalParameterList" => "identifier"),
        prod!("formalParameterList" => "identifier", "comma", "formalParameterList"),

        // Block
        prod!("block" => "openBrace", "closeBrace"),
        prod!("block" => "openBrace", "statementList", "closeBrace"),

        // Expressions
        prod!("expressionStatement" => "expressionSequence", "eos"),
        prod!("expressionSequence" => "singleExpression"),
        prod!("expressionSequence" => "singleExpression", "comma", "expressionSequence"),
        
        // Basic expressions
        prod!("singleExpression" => "identifier"),
        prod!("singleExpression" => "literal"),
        prod!("singleExpression" => "arrayLiteral"),
        prod!("singleExpression" => "objectLiteral"),
        prod!("singleExpression" => "singleExpression", "plus", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "minus", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "multiply", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "divide", "singleExpression"),

        // Control structures
        prod!("ifStatement" => "if", "openParen", "expressionSequence", "closeParen", "statement"),
        prod!("ifStatement" => "if", "openParen", "expressionSequence", "closeParen", "statement", "else", "statement"),

        // Loops
        prod!("iterationStatement" => "while", "openParen", "expressionSequence", "closeParen", "statement"),
        prod!("iterationStatement" => "for", "openParen", "variableStatement", "expressionSequence", "semicolon", "expressionSequence", "closeParen", "statement"),

        prod!("singleExpression" => "identifier"),
        prod!("singleExpression" => "literal"),
        prod!("singleExpression" => "arrayLiteral"),
        prod!("singleExpression" => "objectLiteral"),
        
        // 算术运算符
        prod!("singleExpression" => "singleExpression", "plus", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "minus", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "multiply", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "divide", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "modulus", "singleExpression"),
        prod!("singleExpression" => "minus", "singleExpression"),  // 负号
        prod!("singleExpression" => "plus", "singleExpression"),   // 正号
        
        // 比较运算符
        prod!("singleExpression" => "singleExpression", "lessThan", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "moreThan", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "lessEqual", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "moreEqual", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "equals", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "notEquals", "singleExpression"),
        prod!("singleExpression" => "singleExpression", "identityEquals", "singleExpression"),    // ===
        prod!("singleExpression" => "singleExpression", "identityNotEquals", "singleExpression"), // !==
        
        // 逻辑运算符
        prod!("singleExpression" => "singleExpression", "and", "singleExpression"),  // &&
        prod!("singleExpression" => "singleExpression", "or", "singleExpression"),   // ||
        prod!("singleExpression" => "not", "singleExpression"),                      // !
        
        // 位运算符
        prod!("singleExpression" => "singleExpression", "bitwiseAnd", "singleExpression"),  // &
        prod!("singleExpression" => "singleExpression", "bitwiseOr", "singleExpression"),   // |
        prod!("singleExpression" => "singleExpression", "bitwiseXor", "singleExpression"),  // ^
        prod!("singleExpression" => "bitwiseNot", "singleExpression"),                      // ~
        prod!("singleExpression" => "singleExpression", "leftShift", "singleExpression"),   // <<
        prod!("singleExpression" => "singleExpression", "rightShift", "singleExpression"),  // >>
        
        // 赋值运算符
        prod!("singleExpression" => "singleExpression", "assign", "singleExpression"),           // =
        prod!("singleExpression" => "singleExpression", "plusAssign", "singleExpression"),      // +=
        prod!("singleExpression" => "singleExpression", "minusAssign", "singleExpression"),     // -=
        prod!("singleExpression" => "singleExpression", "multiplyAssign", "singleExpression"),  // *=
        prod!("singleExpression" => "singleExpression", "divideAssign", "singleExpression"),    // /=
        
        // 自增自减
        prod!("singleExpression" => "increment", "singleExpression"),  // ++x
        prod!("singleExpression" => "decrement", "singleExpression"),  // --x
        prod!("singleExpression" => "singleExpression", "increment"),  // x++
        prod!("singleExpression" => "singleExpression", "decrement"),  // x--

        // 类相关语法
        prod!("classDeclaration" => "class", "identifier", "classTail"),
        prod!("classDeclaration" => "class", "identifier", "extends", "singleExpression", "classTail"),
        
        prod!("classTail" => "openBrace", "closeBrace"),
        prod!("classTail" => "openBrace", "classElements", "closeBrace"),
        
        prod!("classElements" => "classElement"),
        prod!("classElements" => "classElement", "classElements"),
        
        prod!("classElement" => "methodDefinition"),
        prod!("classElement" => "staticMethodDefinition"),
        prod!("classElement" => "fieldDefinition"),
        prod!("classElement" => "staticFieldDefinition"),
        prod!("classElement" => "constructor"),
        
        // 构造函数
        prod!("constructor" => "constructor", "openParen", "closeParen", "block"),
        prod!("constructor" => "constructor", "openParen", "formalParameterList", "closeParen", "block"),
        
        // 方法定义
        prod!("methodDefinition" => "identifier", "openParen", "closeParen", "block"),
        prod!("methodDefinition" => "identifier", "openParen", "formalParameterList", "closeParen", "block"),
        
        // 静态方法
        prod!("staticMethodDefinition" => "static", "identifier", "openParen", "closeParen", "block"),
        prod!("staticMethodDefinition" => "static", "identifier", "openParen", "formalParameterList", "closeParen", "block"),
        
        // 字段定义
        prod!("fieldDefinition" => "identifier", "semicolon"),
        prod!("fieldDefinition" => "identifier", "assign", "singleExpression", "semicolon"),
        
        // 静态字段
        prod!("staticFieldDefinition" => "static", "identifier", "semicolon"),
        prod!("staticFieldDefinition" => "static", "identifier", "assign", "singleExpression", "semicolon")
        
        // End of statement
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