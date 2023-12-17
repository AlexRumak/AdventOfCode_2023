use std::{fs};

// Initialize array of strings for one, two, three ... nine
const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let sum = sum_digits(contents); 

    println!("Answer: {sum}");
}

fn sum_digits(contents: String) -> u32 {
    let mut sum = 0;
    for line in contents.lines() {

        let line_str = line.to_string();
        let forward_iter = line_str.chars();
        let mut first_char: u32 = 0;
        'outer: for (i, item) in forward_iter.enumerate() {
            if item.is_ascii_digit() {
                first_char = match item.to_digit(10) {
                    Some(val) => val,
                    None => panic!("Should be an ascii digit...")
                };
                break;
            }

            let slice = &line_str[i..];
            for (j, str) in DIGITS.iter().enumerate() {
                if slice.starts_with(str) {
                    first_char = match j.try_into() {
                        Ok(val) => val,
                        Err(_) => panic!("Should be able to convert to u32"),
                    };
                    first_char += 1;
                    break 'outer;
                }
            }
        }

        let reverse_iter = line_str.chars().rev();
        let mut second_char: u32 = 0;
        'outer: for (i, item) in reverse_iter.enumerate() {
            if item.is_ascii_digit() {
                second_char = match item.to_digit(10) {
                    Some(val) => val,
                    None => panic!("Should be an ascii digit..."),
                };
                break;
            }

            let line_len = line_str.len();

            let slice = &line_str[..line_len - i];
            for (j, str) in DIGITS.iter().enumerate() {
                if slice.ends_with(str) {
                    second_char = match j.try_into(){   
                        Ok(val) => val,
                        Err(_) => panic!("Should be able to convert to u32"),
                    };
                    second_char += 1;
                    break 'outer;
                }
            }
        }

        sum += first_char * 10 + second_char;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sums_correctly() {
        let contents = String::from("a322g\nb3da34fas8sadfasd\nasdfa2sdf22");
        let sum = sum_digits(contents);
        assert_eq!(sum, 32 + 38 + 22);
    }

    #[test]
    fn it_sums_correctly_two() {
        let contents = String::from("onetwo");
        let sum = sum_digits(contents);
        assert_eq!(sum, 12);
    }
}