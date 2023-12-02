// AOC day 2
// Cubes enum with values red, green, or blue
// A bag is filled with a secret number of cubes of each colour every time we play a game
// We play several games and record the information from each gameby ID, followed by
// semicolon-seperated lists of subsets of cubes that were revealed from the bag
// An example input
/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/
// What games are possible if our input is 12 red, 13 green, and 14 blue?
// Sum the ID of the games that are possible given the bag input params
// The result of the example is 8

// Write a rust cli app to solve this
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    game_number: u32,
    colors: Vec<Color>,
}

fn main() -> io::Result<()> {
    //Change the fn inputs to take args red,green, blue, as well as the file
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Please provide a filename red_limit green_limit blue_limit as a command line argument.");
        return Ok(());
    }
    let filename = &args[1];
    let red_limit: u32 = args[2].parse().unwrap_or(0);
    let green_limit: u32 = args[3].parse().unwrap_or(0);
    let blue_limit: u32 = args[4].parse().unwrap_or(0);

    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Process each line
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let game = process_line(&line);
        let sum = sum_colors(&game.colors);
        if sum.red <= red_limit && sum.green <= green_limit && sum.blue <= blue_limit {
            result.push(game.game_number);
        }
    }
    println!("Result: {:?}", result);
    let sum: u32 = result.iter().sum();
    println!("Sum: {:?}", sum);

    Ok(())
}

fn process_line(line: &str) -> Game {
    let mut game = Game {
        game_number: 0,
        colors: Vec::new(),
    };

    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() == 2 {
        game.game_number = parts[0]
            .trim()
            .trim_start_matches("Game")
            .trim()
            .parse()
            .unwrap_or(0);
        let moves = parts[1].split(';');
        for mv in moves {
            let colors: Vec<&str> = mv.split(',').collect();
            let mut color = Color {
                red: 0,
                green: 0,
                blue: 0,
            };
            for c in colors {
                let c = c.trim();
                if c.ends_with("red") {
                    color.red += c.trim_end_matches("red").trim().parse().unwrap_or(0);
                } else if c.ends_with("green") {
                    color.green += c.trim_end_matches("green").trim().parse().unwrap_or(0);
                } else if c.ends_with("blue") {
                    color.blue += c.trim_end_matches("blue").trim().parse().unwrap_or(0);
                }
            }
            game.colors.push(color);
        }
    }
    game
}

// Input is int values red, green, blue
// First load the text file
// Make an int result = 0
// Sum the red, green, and blue squares from each row
fn sum_colors(colors: &[Color]) -> Color {
    let mut sum = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    for color in colors {
        sum.red += color.red;
        sum.green += color.green;
        sum.blue += color.blue;
    }

    sum
}
// If the sum of any value is less or equal to input params, add it to the result
// Return the result
