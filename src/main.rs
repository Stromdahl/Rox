mod expression;
mod lexer;
mod parser;
mod token;

const EX_USAGE: i32 = 64;

const PREFIX: &str = ">";

fn run_file(file_path: &String) {
    println!("->> FILE MODE\n");
    let source =
        std::fs::read_to_string(file_path).expect("Should have been able to read the file");
    run(&source);
}

fn run_prompt() {
    loop {
        print!("{PREFIX} ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        let mut source = String::new();
        std::io::stdin().read_line(&mut source).unwrap(); //TODO: remove unwrap
        println!(" {source}");
        run(&source);
    }
}

fn print_usage() {
    println!("Usage: jrox [script]");
    std::process::exit(EX_USAGE);
}

fn run(source: &str) {
    let chars = source.chars();
    let mut tokens = lexer::Lexer::from_iter(chars).peekable();
    let _expression = crate::parser::parse(&mut tokens);
    todo!()
}

fn main() {
    println!("->> Welcome to Rox!");
    let args: Vec<String> = std::env::args().collect();

    match args.len().cmp(&2) {
        std::cmp::Ordering::Greater => print_usage(),
        std::cmp::Ordering::Equal => run_file(&args[1]),
        std::cmp::Ordering::Less => run_prompt(),
    }
}
