use std::{collections::HashMap, fs, time::SystemTime};

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file input.txt");

    let universe = Universe::new(&contents);
    let points = universe.get_points();

    let now = SystemTime::now();
    println!("Answer: {}", sum_distances(&points));
    println!("Time Elapsed: {:?}", now.elapsed().expect("Should have time"));
}

fn sum_distances(points: &Vec<(i64, i64)>) -> u64 {
    let mut sum = 0;
    for (i, val) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            sum += get_distance(val, &points[j]);
        }
    }
    sum
}

fn get_distance(p1: &(i64, i64), p2: &(i64, i64)) -> u64 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u64
}

struct Universe {
    map: Vec<Vec<char>>,
}

impl Universe {
    fn new(contents: &str) -> Universe {
        let map = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Universe { map }
    }

    fn get_points(&self) -> Vec<(i64, i64)> {
        let mut ret_vec: Vec<(i64, i64)> = Vec::new();
        let mut precomputed_horizontal: HashMap<usize, i64> = HashMap::new();
        let mut precomputed_vertical: HashMap<usize, i64> = HashMap::new();

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                let horizontal_distance = self.get_horizontal_distance(i, &mut precomputed_horizontal);
                let vertical_distance = self.get_vertical_distance(j, &mut precomputed_vertical);
                if self.map[i][j] == '#' {
                    ret_vec.push((
                        horizontal_distance,
                        vertical_distance,
                    ));
                }
            }
        }

        ret_vec
    }

    // Horizontal distance refers to horizontal slices
    // e.g.
    // ..............#............ 0
    // ........................... 1
    //
    // We want to precompute horizontal distances so we don't continually have to expand the entire distance (O(n^2))
    // We couldn't do this with just an iterator that keeps track of the distance? We could, but it would require recomputing values
    // A lot
    fn get_horizontal_distance(
        &self,
        i: usize,
        precomputed_horizontal: &mut HashMap<usize, i64>,
    ) -> i64 {
        match precomputed_horizontal.get(&i) {
            Some(h) => *h,
            None => {
                let distance =if i == 0 {
                    if self.horizontal_contains_galaxy(i) {
                        1
                    } else {
                        1000000
                    }
                } else {
                    precomputed_horizontal
                        .get(&(i - 1))
                        .expect("We iterate through priors, should exist")
                        + if self.horizontal_contains_galaxy(i) {
                            1
                        } else {
                            1000000
                        }
                };

                precomputed_horizontal.insert(i, distance);
                distance
            }
        }
    }

    fn horizontal_contains_galaxy(&self, i: usize) -> bool {
        for j in 0..self.map[i].len() {
            if self.map[i][j] == '#' {
                return true;
            }
        }
        false
    }

    // Vertical distance refers to vertical slices
    // e.g.
    // 0 1
    // . .
    // . .
    // . #
    // . .
    fn get_vertical_distance(
        &self,
        j: usize,
        precomputed_vertical: &mut HashMap<usize, i64>,
    ) -> i64 {
        match precomputed_vertical.get(&j) {
            Some(h) => *h,
            None => {
                let distance = if j == 0 {
                    if self.horizontal_contains_galaxy(j) {
                        1
                    } else {
                        1000000
                    }
                } else {
                    precomputed_vertical
                        .get(&(j - 1))
                        .expect("We iterate through priors, should exist")
                        + if self.vertical_contains_galaxy(j) {
                            1
                        } else {
                            1000000
                        }
                };

                precomputed_vertical.insert(j, distance);
                distance
            }
        }
    }

    fn vertical_contains_galaxy(&self, j: usize) -> bool {
        for i in 0..self.map.len() {
            if self.map[i][j] == '#' {
                return true;
            }
        }
        false
    }
}
