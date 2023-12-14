use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let forward_iter = line.chars();
        let reverse_iter = line.chars().rev();

        let mut first_char: Option<char> = None;
        let mut sec_char: Option<char> = None;
        let mut first_digit_index = 0; 
        for el in forward_iter {
            if el.is_digit(10) {
                first_char = Some(el);
                break;
            }
            first_digit_index += 1;
        }
        
        let mut sec_digit_index = 0;
        for el in reverse_iter {
            if el.is_digit(10) {
                sec_char = Some(el);
                break;
            }
            sec_digit_index += 1;
        }
    }
}