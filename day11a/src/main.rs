use std::{fs, fmt, time::SystemTime};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file input.txt");

    let mut universe = Universe::new(&contents);

    let now = SystemTime::now();
    universe.expand_universe();
    println!("{:?}", now.elapsed().expect("Should have elapsed"));

    let points = universe.get_points();
    println!("Answer: {}", sum_distances(&points));
}

fn sum_distances(points: &Vec<(i32, i32)>) -> u32 {
    let mut sum = 0;
    for (i, val) in points.iter().enumerate() {
        for j in i+1..points.len() {
            sum += get_distance(val, &points[j]);
        }
    }
    sum
}

fn get_distance(p1: &(i32, i32), p2: &(i32, i32)) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1-p2.1).abs()) as u32
}

#[derive(Eq)]
struct Universe {
    map: Vec<Vec<char>>,
}

impl PartialEq for Universe {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl Universe {

    fn new(contents: &str) -> Universe {
        let map = contents.lines().map(|line| line.chars().collect()).collect();
        Universe { map }
    }

    fn expand_universe(&mut self) {
        self.expand_horizontally();
        self.expand_vertically();
    }

    fn expand_vertically(&mut self) {
        let mut new_universe: Vec<Vec<char>> = Vec::new();
        // expand vertically
        for j in 0..self.width() {
            let mut contains_galaxy = false;
            for i in 0..self.height() {
                if self.map[i][j] == '#' {
                    contains_galaxy = true;
                    break;
                }
            }

            for i in 0..self.height() {
                if new_universe.len() < i + 1 {
                    new_universe.push(Vec::new());
                }
                new_universe[i].push(self.map[i][j]);
                if !contains_galaxy {
                    new_universe[i].push(self.map[i][j]);
                }
            }
        }

        self.map = new_universe;
    }

    fn expand_horizontally(&mut self) {
        let mut new_universe: Vec<Vec<char>> = Vec::new();
        // expand horizontal slices
        // iterate horizontally, if encounter no galaxies, duplicate the line
        for i in 0..self.height() {
            let mut contains_galaxy = false;
            for j in 0..self.width() {
                if self.map[i][j] == '#' {
                    contains_galaxy = true;
                    break;
                }
            }

            let new_line = self.map[i].to_vec();
            new_universe.push(new_line.to_owned());
            if !contains_galaxy {
                // duplicate the line
                new_universe.push(new_line.to_owned());
            }
        }
        self.map = new_universe;
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn get_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.map[i][j] == '#' {
                    points.push((i as i32, j as i32));
                }
            }
        }
        points
    }
}

impl fmt::Debug for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for line in &self.map {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_universe() {
        let mut galaxy = Universe::new("#..\n...\n..#");
        let expanded_galaxy = Universe::new("#...\n....\n....\n...#");

        galaxy.expand_universe();

        assert_eq!(galaxy, expanded_galaxy);
    }

    #[test]
    fn test_get_points() {
        let mut galaxy = Universe::new("#..\n...\n..#");

        galaxy.expand_universe();
        let points = galaxy.get_points();

        assert!(points.contains(&(0, 0)));
        assert!(points.contains(&(3, 3)));
    }

    #[test]
    fn test_get_distance() {
        let mut galaxy = Universe::new("#..\n...\n..#");

        galaxy.expand_universe();
        let points = galaxy.get_points();

        assert_eq!(sum_distances(&points), 6);
    }
}