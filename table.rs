use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut variables = Vec::new();
    let mut trace_table = Vec::new();
    let contents = fs::read_to_string(&args[1]).expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();

    for line in &lines {
        let statement = line.trim();

        if statement.starts_with("//") {
            continue;
        }

        if statement.contains("let") {
            let parts: Vec<&str> = statement.split("=").map(|x| x.trim()).collect();
            variables.push(parts[0].replace("let ", ""));
            trace_table.push(parts[1].replace(";", ""));
        }
        println!("{}", statement);
    }

    println!();
    let header = variables.join("\t");
    println!("{}", header);
    let row = trace_table.join("\t");
    println!("{}", row);
}