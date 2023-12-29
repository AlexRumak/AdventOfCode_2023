use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        let els: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().expect("Should be able to parse into u64")).collect();
        let answer = get_next_element(els);
        sum += answer.expect("Should have an answer");
    }
    println!("Sum: {}", sum);
}

fn get_next_element(els: Vec<i64>) -> Option<i64> {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(els);
    while vecs.last()?.iter().any(|x| *x != 0) {
        let new_vec: Vec<i64> = vecs.last()?
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect();

        vecs.push(new_vec);
    }
    
    let mut new_first : i64 = 0;
    for x in (0..vecs.len() - 1).rev() {
        new_first = vecs[x].first()? - new_first;
    }

    Some(new_first)
}
