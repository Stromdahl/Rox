mod lexer;
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
    println!("->> REPL MODE\n");
    let mut source = String::new();
    loop {
        source.clear();
        std::io::stdin().read_line(&mut source).unwrap(); //TODO: remove unwrap
        print!("{PREFIX}");
        run(&source);
    }
}

fn run(source: &String) {
    let chars = source.chars();
    let tokens: Vec<token::Token> = lexer::Lexer::from_iter(chars).collect();
    for token in tokens {
        println!("{token:?}");
    }
}

fn main() {
    print!("--> STARTING ROX -- ");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: jrox [script]");
        std::process::exit(EX_USAGE);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
