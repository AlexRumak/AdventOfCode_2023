use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file input.txt");

    let parts_map = parse_input(contents)
        .expect("Something went wrong parsing the input");

    let mut sum = 0;
    for (y, line) in parts_map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            sum += return_parts_num_value(x, y, &parts_map);
        }
    }

    print!("Sum of parts: {}", sum);
}

fn return_parts_num_value(x: usize, y: usize, map: &Vec<Vec<char>>) -> u32 {
    if !map[y][x].is_ascii_digit() {
        return 0;
    }

    if x > 0 && map[y][x-1].is_ascii_digit() {
        return 0;
    }

    return part_value(x, y, &map)
}

/// Assumptions:
/// - x and y are valid coordinates
/// - x and y coordinates are the starting coordinates of the number
fn part_value(x: usize, y: usize, map: &Vec<Vec<char>>) -> u32 {
    let mut i = x;
    let mut number: Vec<char> = Vec::new();

    let mut is_adjacent = false;
    while i < map[y].len() && map[y][i].is_ascii_digit() {
        number.push(map[y][i]);
        if are_adjacent_to_part(i, y, &map) {
            is_adjacent = true;
        }
        i += 1;
    }

    if !is_adjacent {
        return 0;
    }

    let number_str: String = number.into_iter().collect();

    number_str.parse::<u32>()
        .expect("Not a valid number")
}

fn are_adjacent_to_part(x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    return x != 0 && is_part(x-1, y, &map)
        || y != 0 && is_part(x, y-1, &map)
        || is_part(x+1, y, map)
        || is_part(x, y+1, map)
        || x != 0 && y != 0 && is_part(x-1, y-1, map)
        || is_part(x+1, y+1, map)
        || x != 0 && is_part(x-1, y+1, map)
        || y != 0 && is_part(x+1, y-1, map)
}

fn is_part(x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    // guards
    if x >= map[0].len() || y >= map.len() {
        return false;
    }
    return !map[y][x].is_ascii_digit() && map[y][x] != '.'
}

fn print_map(map: Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn parse_input(contents: String) -> Option<Vec<Vec<char>>> {
    // convert string to vector of vectors
    let mut vec: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        vec.push(line.chars().collect())
    }

    Some(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_works() {
        let contents = String::from("..123\n.....\n.#2..\n12...\n...@4");
        let parts_map = parse_input(contents)
            .expect("Something went wrong parsing the input");
        assert_eq!(parts_map[0][2], '1');
        assert_eq!(parts_map[1][0], '.');
    }
    
    #[test]
    fn is_number_adjacent_to_part_works()
    {
        let contents = String::from("..123\n.....\n.#2..\n12...\n...@4");
        let parts_map = parse_input(contents)
            .expect("Something went wrong parsing the input");
        assert_eq!(part_value(2, 0, &parts_map), 0);
        assert_eq!(part_value(2, 2, &parts_map), 2);
    }
}
