use std::{fs, time::SystemTime};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should be able to read the file");

    let now = SystemTime::now();
    let mut sum = 0;
    for line in contents.lines() {
        let mut iter = line.split(' ');

        let combination_str = iter.next().expect("Should exist");
        let grouping_str = iter.next().expect("Should exist");
        
        sum += get_possible_combinations(combination_str, grouping_str);
    }
    println!("Answer: {}", sum);
    println!("Time Elapsed: {:?}", now.elapsed().expect("Elapsed time should exist"));
}

// The input is pretty short, we don't have to memoize it or whatever
// Which was true, it took 26 seconds without memoization or a map
// Let's try it with memoization
fn get_possible_combinations(str: &str, grouping_str: &str) -> u64 {
    if !str.contains('?') {
        return if get_grouping_string(str) == grouping_str { 1 } else { 0 } // if combination is invalid, don't allow it
    }
    
    let mut sum = 0;
    sum += get_possible_combinations(&str.replacen('?', ".", 1), grouping_str);
    sum += get_possible_combinations(&str.replacen('?', "#", 1), grouping_str);
    sum
}

fn get_grouping_string(str: &str) -> String {
    let mut groupings: Vec<String> = Vec::new();
        
    let mut index = 0;
    for char in str.chars() {
        if char == '#' {
            index += 1;
            continue;
        }

        if index > 0 {
            groupings.push(index.to_string());
            index = 0;
        }
    }

    if index > 0 {
        groupings.push(index.to_string());
    }
    
    groupings.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_amount() {
        let input = "??###.##.";
        let grouping_str = "1,3,2";
        assert_eq!(get_possible_combinations(input, grouping_str), 1);
    }

    #[test]
    fn test_combination_amount_2() {
        let input = "?????.##.";
        let grouping_str = "1,2,2";
        assert_eq!(get_possible_combinations(input, grouping_str), 3);
    }

    #[test]
    fn test_find_grouping() {
        let input = "#.###..##.";
        assert_eq!(get_grouping_string(input), "1,3,2");

        let input = "#.###..##..#";
        assert_eq!(get_grouping_string(input), "1,3,2,1");

        let input = ".#.###..##..#";
        assert_eq!(get_grouping_string(input), "1,3,2,1");
    }
}
