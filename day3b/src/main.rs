use std::fs;
use std::collections::HashSet;

const DIRECTIONS : [[i32; 2]; 8]= [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have read input.txt");

    let map = parse(input);

    let sum = calculate_answer(map);

    println!("Sum: {}", sum);
    println!("");
}

fn calculate_answer(map: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                sum += calculate_gear_ratio(x, y, &map);
            }
        }
    }
    sum
}

fn parse(input: String) -> Vec<Vec<char>> {
    let map : Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    map
}

fn calculate_gear_ratio(x: usize, y: usize, map: &Vec<Vec<char>>) -> u32
{
    // look for adjacent numbers
    let mut adjacent_numbers: HashSet<Vec<(usize, usize)>> = HashSet::new();
    
    for direction in DIRECTIONS.iter() {
        let i = x as i32 + direction[0];
        let j = y as i32 + direction[1];

        if i < 0 || j < 0 {
            continue;
        }
        
        if j >= map.len() as i32 || i >= map[j as usize].len() as i32 {
            continue;
        }

        if map[j as usize][i as usize].is_numeric() {
            adjacent_numbers.insert(get_number(i as usize, j as usize, &map));
        }
    }

    if adjacent_numbers.len() != 2 {
        return 0;
    }

    let mut iter = adjacent_numbers.iter();
    let first_number = iter.next()
        .expect("First number");
    let second_number = iter.next()
        .expect("Second number");

        get_value(first_number, &map) * get_value(second_number, &map)
}

fn get_value(number: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> u32 {
    let mut value = 0;
    for (x, y) in number {
        value *= 10;
        value += map[*y][*x].to_digit(10).expect("Digit");
    }
    value
}

fn get_number(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut i = x;

    // look left until we get the start of the number
    loop {
        if i == 0 {
            break; // We found the start of the number
        }

        if (map[y][i - 1]).is_numeric() {
            i -= 1;
        }
        else {
            break;
        }
    }

    let mut number: Vec<(usize, usize)> = Vec::new();
    loop {
        if i >= map[y].len() {
            break; // We found the end of the number
        }

        if (map[y][i]).is_numeric() {
            number.push((i, y));
            i += 1;
        }
        else {
            break;
        }
    }

    number
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gear_ratio() {
        let map_str = "\
        .12.3.\n\
        ...234\n\
        12*...";

        let map = parse(map_str.to_string());

        assert_eq!(calculate_gear_ratio(2, 2, &map), 12 * 234)
    }

    #[test]
    fn test_calculate_gear_ratio_two() {
        let map_str = "\
        .12.3.\n\
        ...12.\n\
        12*...";

        let map = parse(map_str.to_string());

        assert_eq!(calculate_gear_ratio(2, 2, &map), 144)
    }

    #[test]
    fn test_calculate_answer() {
        let map_str = "\
        .12*3.\n\
        ...234\n\
        12*...";

        let map = parse(map_str.to_string());

        assert_eq!(calculate_answer(map), 12 * 234)
    }
}