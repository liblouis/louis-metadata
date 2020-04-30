use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Problem parsing arguments: not enough arguments");
        process::exit(1);
    }

    let filename = args[1].clone();
    
    if let Err(e) = louis_grep::run(filename) {
        println!("Application error: {}", e);
        process::exit(1);
    }}

