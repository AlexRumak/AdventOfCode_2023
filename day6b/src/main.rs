use std::fs;

struct RaceRecord {
    time: u64,
    distance: u64
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should be able to read from file input.txt");

    let race_records = parse_race_records(contents);

    print_race_records(&race_records);

    let mut answers: Vec<u64> = Vec::new();
    race_records.iter().for_each(|record| {
        answers.push(compute_answer(record));
    });

    let answer: u64 = answers.iter().product();
    println!("answer: {}", answer);
}

// td = speed(tl)
// tt = tl + tp
// tl = tt - tp
// speed = tp
// td = tp(tt-tp)
// 0 = -tp^2 + tttp - td
// tp = (-tt +- srqt(tt^2 + 4*(-td))) / (-2)
fn compute_answer(race_record: &RaceRecord) -> u64 {
    let total_time = race_record.time as f64;
    let record = race_record.distance as f64;

    let lower_bound = (-total_time + f64::sqrt(total_time.powf(2.0) - 4.0 * record)) / -2.0;
    let upper_bound = (-total_time - f64::sqrt(total_time.powf(2.0) - 4.0 * record)) / -2.0;

    println!("lower_bound: {}, upper_bound: {}", lower_bound, upper_bound);

    let lb = lower_bound.ceil() as u64;
    let ub = upper_bound.floor() as u64;
    ub - lb + 1
}

fn parse_race_records(contents: String) -> Vec<RaceRecord> {
    let mut lines = contents.lines();
    let mut times = lines.next()
        .expect("Should get first line")
        .split(' ')
        .filter(|&s| !s.is_empty())
        .peekable();
    let mut distances = lines.next()
        .expect("Should get second line")
        .split(' ')
        .filter(|&s| !s.is_empty())
        .peekable();

    // skip labels
    times.next();
    distances.next();
    
    let mut time_string = String::new();
    let mut distance_string = String::new();

    while times.peek().is_some() || distances.peek().is_some() {
        if times.peek().is_some() != distances.peek().is_some() {
            panic!("There are a different number of times and distances in the input");
        }

        time_string.push_str(times.next().expect("Should have a time"));
        distance_string.push_str(distances.next().expect("Should have a distance"));
    }


    let mut race_records: Vec<RaceRecord> = Vec::new();

    race_records.push(
        RaceRecord {
            time: time_string.parse::<u64>().expect("Should parse"),
            distance: distance_string.parse::<u64>().expect("Should parse"),
        }
    );

    race_records
}

fn print_race_records(race_records: &Vec<RaceRecord>) {
    for (i, race_record) in race_records.iter().enumerate() {
        println!("Race {}: time: {}, distance: {}", i, race_record.time, race_record.distance);
    }
}
