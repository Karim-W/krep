use std::io::{self, BufRead};
use colored::Colorize;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <pattern>", args[0]);
        return;
    }
    let pattern = &args[1];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("nothing to read");
        let split: Vec<&str> = line.split(pattern).collect();
        if split.len() > 1 {
            println!("{}", split.join(&pattern.red().to_string()));
        } 
    }
}
