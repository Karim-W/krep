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
    let mut matched_lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("nothing to read");
        if line.contains(pattern) {
            matched_lines.push(line);
        }
    }
    for line in matched_lines {
        let split = line.split(pattern);
        println!("{}", 
         split.collect::<Vec<&str>>().
         join(&format!("{}", pattern.red().bold().underline())));
    }
}
