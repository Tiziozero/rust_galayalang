use std::fs::read_to_string;

mod lexer;
mod parser;
mod symbols;
mod type_check;


fn main()  {
    let code = read_to_string("main.gala").unwrap();
    let lexer = lexer::Lexer::from_code(&code).unwrap_or_else(|err| {
        panic!("error {}", err);});
    println!("end");
    let p = parser::Parser::parse(lexer); // takes ownership
    match &p.root {
        Some(b) =>
            for s in b {
                println!("Parser root: {}", s);
            }
        _ => panic!("Failed to parse"),
    }
    let resolved = symbols::Resolver::new(&p).unwrap();
    let errs = type_check::TypeChecker::resolve(&resolved, &p);
}


