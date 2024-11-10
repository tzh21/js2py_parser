//! usage: ./{{project_name}} <filename>


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if !std::path::Path::new(filename).exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(filename).expect("Failed to read file");
    let token_stream = toy_lang_lexer::lexer::token_stream(&input);
    for token in token_stream {
        println!("('{:?}', '{}')", token.typ, token.val);
    }
}
