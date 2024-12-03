mod day_1;

use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: <day_number>");
        std::process::exit(1);
    }

    // Get the day number from the arguments
    let day_input = &args[1];
    let mut file = File::open(format!("src/{}_puzzle.txt", day_input))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Running {}", day_input);

    // Match the day number and run the corresponding module
    match day_input.as_str() {
        "day_1" => {
            println!("Day_1 part 1 solution: {}", day_1::run_part_1(contents.clone()));
            println!("Day_2 part 2 solution: {}", day_1::run_part_2(contents.clone()));
        }
        // Add more cases as needed
        _ => {
            eprintln!("Invalid day number: {}", day_input);
            std::process::exit(1);
        }
    }

    Ok(())
}
