use std::env;
use std::fs;

const EX_USAGE: i32 = 64;

fn run_file(file_path: &String) {

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn run_prompt() {
    println!("run prompt ");
    todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("Usage: jrox [script]");
        std::process::exit(EX_USAGE);
    } else if args.len() == 2{
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
// use std::env;
// 
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);
// }
