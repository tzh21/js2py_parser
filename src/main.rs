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
        //
        prod!("importStatement" => "import", "importFromBlock"),
        prod!("importFromBlock" => "str", "eos"),
        prod!("exportStatement" => "export", "exportClause", "eos"),        //export
        prod!("exportClause" => "exportedElementList"),
        prod!("exportedElementList" => "identifier", "exportedElementList"),
        prod!("exportedElementList" => "identifier"),
        prod!("eos" => "eof"),
        prod!("eos" => "semicolon"),
        //var
        prod!("variableStatement" => "varDeclaration"),
        prod!("variableStatement" => "letDeclaration"),
        prod!("variableStatement" => "constDeclaration"),
        prod!("varDeclaration" => "var", "variableDeclarationList", "eos"),
        prod!("letDeclaration" => "let", "variableDeclarationList", "eos"),
        prod!("constDeclaration" => "const", "variableDeclarationList", "eos"),
        prod!("variableDeclarationList" => "variableDeclaration", "comma", "variableDeclarationList"),
        prod!("variableDeclarationList" => "variableDeclaration"),
        prod!("variableDeclaration" => "identifier", "initializer_"),                      //initializer_ => (optional) initializer
        prod!("initializer_" => "=", "expression"),
        prod!("initializer_" => ""),
        //function
        prod!("statement" => "functionExpression"),
        prod!("functionExpression" => "function", "identifier_", "parameters", "block"),   //identifier_ => (optional) identifier
        prod!("functionExpression" => "arrowFunction"),
        prod!("arrowFunction" => "parameters", "=>", "expression"),
        prod!("parameters" => "(", "parameterList", ")"),
        prod!("parameterList" => "identifier", "comma", "parameterList"),
        prod!("parameterList" => "identifier"),
        prod!("parameterList" => ""),
        //class
        prod!("classDeclaration" => "class", "identifier", "classBody"),
        prod!("classBody" => "{", "classElementList", "}"),
        prod!("classElementList" => "classElement", "classElementList"),
        prod!("classElementList" => "classElement"),
        prod!("classElement" => "methodDefinition"),
        prod!("classElement" => "propertyDefinition"),
        prod!("methodDefinition" => "identifier", "parameters", "block"),
        prod!("propertyDefinition" => "identifier", ":", "expression"),
        //deconstructing assignment
        prod!("variableStatement" => "destructuringAssignment"),
        prod!("destructuringAssignment" => "let", "arrayDestructuring", "=", "expression"),
        prod!("destructuringAssignment" => "const", "objectDestructuring", "=", "expression"),
        prod!("arrayDestructuring" => "[", "elementList", "]"),
        prod!("elementList" => "identifier", "comma", "elementList"),
        prod!("elementList" => "identifier"),
        prod!("objectDestructuring" => "{", "propertyList", "}"),
        prod!("propertyList" => "identifier", "comma", "propertyList"),
        prod!("propertyList" => "identifier"),
        //template
        prod!("expression" => "templateLiteral"),
        prod!("templateLiteral" => "`", "templateCharacters", "`"),
        prod!("templateCharacters" => "templateCharacter", "templateCharacters"),
        prod!("templateCharacters" => "templateCharacter"),
        //expression
        prod!("expressionStatement" => "expression", "eos"),
        //if
        prod!("ifStatement" => "if", "(", "expression", ")", "statement", "elseClause_"),
        prod!("elseClause_" => "else", "statement"),
        prod!("elseClause_" => ""),
        //iteration
        prod!("iterationStatement" => "whileStatement"),
        prod!("iterationStatement" => "doWhileStatement"),
        prod!("iterationStatement" => "forStatement"),
        prod!("whileStatement" => "while", "(", "expression", ")", "statement"),
        prod!("doWhileStatement" => "do", "statement", "while", "(", "expression", ")", "eos"),
        //for
        prod!("forStatement" => "for", "(", "forInit_", ";", "forCondition_", ";", "forUpdate_", ")", "statement"),
        prod!("forInit_" => "variableStatement"),
        prod!("forInit_" => "expression"),
        prod!("forInit_" => ""),
        prod!("forCondition_" => "expression"),
        prod!("forCondition_" => ""),
        prod!("forUpdate_" => "expression"),
        prod!("forUpdate_" => ""),
        //continue
        prod!("continueStatement" => "continue", "identifier_", "eos"),
        //break
        prod!("breakStatement" => "break", "identifier_", "eos"),
        //return
        prod!("returnStatement" => "return", "expression_", "eos"),
        prod!("expression_" => "expression"),
        prod!("expression_" => ""),
        //yield
        prod!("yieldStatement" => "yield", "expression_", "eos"),
        //with
        prod!("withStatement" => "with", "(", "expression", ")", "statement"),
        //labelled
        prod!("labelledStatement" => "identifier", ":", "statement"),
        //switch
        prod!("switchStatement" => "switch", "(", "expression", ")", "{", "caseBlock", "}"),
        prod!("caseBlock" => "caseClauses_", "defaultClause_", "caseClauses_"),
        prod!("caseClauses_" => "caseClause", "caseClauses_"),
        prod!("caseClauses_" => ""),
        prod!("caseClause" => "case", "expression", ":", "statementList_"),
        prod!("defaultClause_" => "defaultClause"),
        prod!("defaultClause_" => ""),
        prod!("defaultClause" => "default", ":", "statementList_"),
        prod!("statementList_" => "statementList"),
        prod!("statementList_" => ""),
        //throw
        prod!("throwStatement" => "throw", "expression", "eos"),
        //try
        prod!("tryStatement" => "try", "block", "catchClause_", "finallyClause_"),
        prod!("catchClause_" => "catchClause"),
        prod!("catchClause_" => ""),
        prod!("catchClause" => "catch", "(", "identifier", ")", "block"),
        prod!("finallyClause_" => "finallyClause"),
        prod!("finallyClause_" => ""),
        prod!("finallyClause" => "finally", "block"),
        //debugger
        prod!("debuggerStatement" => "debugger", "eos")
        //expression
        prod!("expression" => "assignmentExpression"),
        prod!("assignmentExpression" => "logicalOrExpression"),
        prod!("assignmentExpression" => "identifier", "=", "logicalOrExpression"),
        prod!("assignmentExpression" => "identifier", "+=", "logicalOrExpression"),
        prod!("assignmentExpression" => "identifier", "-=", "logicalOrExpression"),

        prod!("logicalOrExpression" => "logicalAndExpression"),
        prod!("logicalOrExpression" => "logicalOrExpression", "||", "logicalAndExpression"),
        prod!("logicalAndExpression" => "equalityExpression"),
        prod!("logicalAndExpression" => "logicalAndExpression", "&&", "equalityExpression"),
        prod!("equalityExpression" => "relationalExpression"),
        prod!("equalityExpression" => "equalityExpression", "==", "relationalExpression"),
        prod!("equalityExpression" => "equalityExpression", "!=", "relationalExpression"),
        prod!("relationalExpression" => "additiveExpression"),
        prod!("relationalExpression" => "relationalExpression", "<", "additiveExpression"),
        prod!("relationalExpression" => "relationalExpression", ">", "additiveExpression"),
        prod!("relationalExpression" => "relationalExpression", "<=", "additiveExpression"),
        prod!("relationalExpression" => "relationalExpression", ">=", "additiveExpression"),
        prod!("additiveExpression" => "multiplicativeExpression"),
        prod!("additiveExpression" => "additiveExpression", "+", "multiplicativeExpression"),
        prod!("additiveExpression" => "additiveExpression", "-", "multiplicativeExpression"),
        prod!("multiplicativeExpression" => "unaryExpression"),
        prod!("multiplicativeExpression" => "multiplicativeExpression", "*", "unaryExpression"),
        prod!("multiplicativeExpression" => "multiplicativeExpression", "/", "unaryExpression"),
        prod!("unaryExpression" => "primaryExpression"),
        prod!("unaryExpression" => "-", "unaryExpression"),
        prod!("unaryExpression" => "!", "unaryExpression"),
        prod!("primaryExpression" => "identifier"),
        prod!("primaryExpression" => "literal"),
        prod!("primaryExpression" => "(", "expression", ")")
        //func
        prod!("expression" => "identifier", "(", "argumentList_", ")"),
        prod!("argumentList_" => "expression", ",", "argumentList_"),
        prod!("argumentList_" => "expression"),
        prod!("argumentList_" => ""),
        //obj
        prod!("expression" => "objectLiteral"),
        prod!("objectLiteral" => "{", "propertyList_", "}"),
        prod!("propertyList_" => "property", ",", "propertyList_"),
        prod!("propertyList_" => "property"),
        prod!("propertyList_" => ""),
        prod!("property" => "identifier", ":", "expression"),
        //array
        prod!("expression" => "arrayLiteral"),
        prod!("arrayLiteral" => "[", "elementList_", "]"),
        prod!("elementList_" => "expression", ",", "elementList_"),
        prod!("elementList_" => "expression"),
        prod!("elementList_" => ""),
        //template
        prod!("templateLiteral" => "`", "templateCharacters", "`"),
        prod!("templateCharacters" => "templateCharacter", "templateCharacters"),
        prod!("templateCharacters" => "${", "expression", "}", "templateCharacters"),
        prod!("templateCharacters" => "templateCharacter"),
        prod!("templateCharacters" => ""),
        //array
        prod!("arrowFunction" => "parameters", "=>", "block"),
        prod!("parameters" => "(", "parameterList", ")"),
        prod!("parameterList" => "identifier", "=", "expression", ",", "parameterList"),
        prod!("parameterList" => "identifier", "=", "expression"),
        prod!("parameterList" => "identifier", ",", "parameterList"),
        prod!("parameterList" => "identifier"),
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
