use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to find file input.txt");

    let mut sum = 0;
    for line in contents.lines() {
        sum += get_game_score(line);
    }

    print!("The sum of all games is {}", sum);
}

fn get_game_score(game: &str) -> i32 {
    let mut x = game.split(':');
    x.next(); // Skip first
    let rounds = x.next()
        .expect("Unable to find rounds");

    let mut max_red_seen = 0;
    let mut max_green_seen = 0;
    let mut max_blue_seen = 0;

    for round in rounds.split(';') {
        for cube in round.split(',') {
            let mut sub_round = cube.split(' ');
            sub_round.next(); // Skip first, which will be empty due to formatting
            let amount = sub_round.next()
                .expect("Unable to find amount")
                .parse::<i32>()
                .expect("Unable to parse amount");

            match sub_round.next().expect("There should be a color") {
                "red" => {
                    if amount > max_red_seen {
                        max_red_seen = amount;
                    }
                },
                "green" => {
                    if amount > max_green_seen {
                        max_green_seen = amount;
                    }
                },
                "blue" => {
                    if amount > max_blue_seen {
                        max_blue_seen = amount;
                    }
                },
                color => panic!("Unknown color: {}", color)
            }
        }
    }

    max_blue_seen * max_green_seen * max_red_seen
}