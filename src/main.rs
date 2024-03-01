use is_haiku::count_syllables;
use std::io::{self, BufRead};

fn main() -> Result<(), ()> {
    let stdin = io::stdin();

    let result: Vec<usize> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.expect("Failed to read line from stdin")
                .split(" ")
                .map(count_syllables)
                .sum()
        })
        .collect();

    match result[..] {
        [5, 7, 5] => {
            println!("Provided input is a haiku. Result: {}", true);
            Ok(())
        }
        _ => {
            println!("Provided input is not a haiku. Result: {}", false);
            Err(())
        }
    }
}
