use std::{fs, collections::HashSet};

fn main() {
    
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the file input.txt");

    let mut sum = 0;
    for line in input.lines() {
        let mut game_info = line.split(':');
        game_info.next();
        let game = game_info.next()
            .expect("Should be the game");

        let mut game = game.split('|');
        let mut your_numbers: HashSet<&str> = HashSet::from_iter(game.next()
            .expect("Should be your numbers")
            .split_ascii_whitespace()
            .filter(|&x| !x.is_empty()));

        let mut winning_numbers: HashSet<&str> = HashSet::from_iter(game.next()
            .expect("Should be the winning numbers")
            .split_ascii_whitespace()
            .filter(|&x| !x.is_empty()));

        let matching_numbers = your_numbers.intersection(&winning_numbers);

        let matching_numbers = matching_numbers.count();
        if matching_numbers == 0 {
            sum += 0;
        }
        else {
            let base: u32 = 2;
            sum += base.pow(matching_numbers as u32 - 1);
        }
    }

    println!("Sum: {}", sum);
}
