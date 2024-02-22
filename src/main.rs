use is_haiku::{count_syllables, get_input};

fn main() -> Result<(), ()> {
    let result: Vec<usize> = get_input()
        .lines()
        .map(|line| line.split(" ").map(count_syllables).sum::<usize>())
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
