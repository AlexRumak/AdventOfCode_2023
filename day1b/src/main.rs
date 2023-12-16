use std::fs;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let sum = sum_digits(contents); 

    println!("Answer: {sum}");
}

fn sum_digits(contents: String) -> i32 {
    let mut sum = 0;
    for line in contents.lines() {
        let mut forward_iter = line.chars();
        let mut reverse_iter = line.chars().rev();
        
        let first_char = forward_iter.find(|&c| c.is_ascii_digit());
        let sec_char = reverse_iter.find(|&c| c.is_ascii_digit());
        
        let digits = format!("{}{}", first_char.unwrap(), sec_char.unwrap());
        sum += digits.parse::<i32>().unwrap();
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
}