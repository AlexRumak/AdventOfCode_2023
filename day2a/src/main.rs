use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Reading from file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for game in contents.lines() {
        let mut subgames = game.split(':');
        let game_info_str = subgames.next()
            .expect("First Element Should be Game Info");
        
        let mut game_info = game_info_str.split(' ');
        game_info.next(); // Skip first element which is just 'Game'

        let game_num = game_info.next()
            .expect("Game number should come after Game")
            .parse::<u32>()
            .expect("Should be able to parse Game number into integer");

        let rounds = subgames.next()
            .expect("Second Element Should be Rounds");
            

        let mut game_is_valid = true;
        'outer: for round in rounds.split(';') {
            for cube in round.split(',') {
                let mut iter = cube.split(' ');
                iter.next(); // Skip first element which is empty string lol
                let number = iter.next()
                    .expect("Should be able to get first element")
                    .parse::<u32>()
                    .expect("Should be able to parse");
                let color = iter.next()
                    .expect("Should be able to get second element - color of the cube");

                match color {
                    "red" => {
                        if number > 12 {
                            game_is_valid = false;
                            break 'outer;
                        }
                    },
                    "green" => {
                        if number > 13 {
                            game_is_valid = false;
                            break 'outer;
                        }
                    },
                    "blue" => {
                        if number > 14 {
                            game_is_valid = false;
                            break 'outer;
                        }
                    },
                    x => panic!("Invalid color given: {}", x),
                }
            }
        }

        if game_is_valid {
            sum += game_num;
        }
        else {
            println!("Game {} is invalid. Game Info: {}", game_num, rounds);
        }
    }

    print!("Sum of valid games: {}", sum);
}
