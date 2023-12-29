use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        let els: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().expect("Should be able to parse into u64")).collect();
        let answer = get_next_element(els);
        sum += answer;
    }
    println!("Sum: {}", sum);
}

fn get_next_element(els: Vec<i64>) -> i64 {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(els);
    while vecs.last().expect("There should be a last element").iter().any(|x| *x != 0) {
        let new_vec: Vec<i64> = vecs.last()
            .expect("There should be a last element")
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect();


        vecs.push(new_vec);
    }
    
    for x in (0..vecs.len()).rev() {
        if x == vecs.len() - 1 {
            vecs[x].push(0);
            continue;
        }

        let new_el = vecs[x + 1].last().expect("There should be a last el") + vecs[x].last().expect("There should be a last el");
        vecs[x].push(new_el);
    }

    *vecs.first().expect("Should be a first vec").last().expect("Should be a last element of first vec")
}
