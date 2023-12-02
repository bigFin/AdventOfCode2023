use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a filename as a command line argument.");
        std::process::exit(1);
    }
    let filename = &args[1];

    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Process each line
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        if let Some((first_digit, last_digit)) = extract_digits(&line) {
            let combined_value = combine_digits(first_digit, last_digit);
            sum += combined_value;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn extract_digits(line: &str) -> Option<(u32, u32)> {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.is_empty() {
        return None;
    }

    let first_digit = digits[0];
    let last_digit = digits.last().copied().unwrap_or(first_digit);

    Some((first_digit, last_digit))
}
fn combine_digits(first_digit: u32, last_digit: u32) -> u32 {
    first_digit * 10 + last_digit
}
