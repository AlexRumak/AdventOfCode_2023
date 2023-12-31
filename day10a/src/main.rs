use std::{collections::{HashMap, HashSet}, fs, ops::Add};

use lazy_static::lazy_static;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let map: Map = Map::new(contents);

    let answer = get_furthest_location(&map);

    println!("Answer: {}", answer);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    Invalid
}

impl Direction {
    fn opposite_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Invalid => Direction::Invalid,
        }
    }
}

lazy_static! {
    static ref DIRECTION_TO_MOVEMENT: HashMap<Direction, Point> = {
        let mut m = HashMap::new();
        m.insert(Direction::North, Point::new(0, -1));
        m.insert(Direction::East, Point::new(1, 0));
        m.insert(Direction::South, Point::new(0, 1));
        m.insert(Direction::West, Point::new(-1, 0));
        m
    };
}

// x = index[0], y = index[1]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}



impl<'a> Add<&'a Point> for Point {
    type Output = Point;
    fn add(self, other: &'a Point) -> Point {
        Point {
            x: self.x + &other.x,
            y: self.y + &other.y,
        }
    }
}

struct Map {
    map: Vec<Vec<char>>,
    starting_location: Point,
}

impl Map {
    fn new(map: String) -> Map {
        let internal_map = map.split('\n').map(|line| line.chars().collect()).collect();
        
        let mut ret_val = Map {
            map: internal_map,
            starting_location: Point::new(0, 0),
        };

        for (y, line) in ret_val.map.iter().enumerate() {
            for (x, character) in line.iter().enumerate() {
                if *character == 'S' {
                    ret_val.starting_location = Point::new(x as i32, y as i32);
                    return ret_val;
                }
            }
        }
        
        panic!("Map has no starting location");
    }

    fn get(&self, point: Point) -> char {
        return self.map[point.y as usize][point.x as usize];
    }
}

fn get_furthest_location(map: &Map) -> u32 {
    // Travel in both direction until you have visited all the nodes
    let mut visited_nodes : HashSet<Point> = HashSet::new();
    visited_nodes.insert(map.starting_location.clone());

    // find starting directions
    let (mut first_direction, mut second_direction) = get_starting_directions(map);

    let mut first_pointer = map.starting_location + &DIRECTION_TO_MOVEMENT[&first_direction];
    let mut second_pointer = map.starting_location + &DIRECTION_TO_MOVEMENT[&second_direction];

    visited_nodes.insert(first_pointer.clone());
    visited_nodes.insert(second_pointer.clone());

    let mut distance = 1;
    loop {
        let first_char = map.get(first_pointer);
        first_direction = get_next_direction(first_char, &first_direction.opposite_direction());

        if first_direction == Direction::Invalid {
            panic!("New first direction is invalid");
        }

        let second_char = map.get(second_pointer);
        second_direction = get_next_direction(second_char, &second_direction.opposite_direction());

        if second_direction == Direction::Invalid {
            panic!("New second direction is invalid");    
        }

        first_pointer = first_pointer + &DIRECTION_TO_MOVEMENT[&first_direction];
        second_pointer = second_pointer + &DIRECTION_TO_MOVEMENT[&second_direction];

        if visited_nodes.contains(&first_pointer) {
            break;
        }

        visited_nodes.insert(first_pointer.clone());

        if visited_nodes.contains(&second_pointer) {
            distance += 1;
            break;
        }

        visited_nodes.insert(second_pointer.clone());
        distance += 1;
    }

    distance
}

fn get_starting_directions(map: &Map) -> (Direction, Direction) {
    let mut directions: Vec<Direction> = Vec::new();

    DIRECTION_TO_MOVEMENT.keys().for_each(|direction| {
        let point = &DIRECTION_TO_MOVEMENT[direction];

        let new_x = map.starting_location.x as i32 + point.x;
        let new_y = map.starting_location.y as i32 + point.y;

        if new_x < 0 || new_y < 0 || new_x >= map.map[0].len() as i32 || new_y >= map.map.len() as i32 {
            return;
        }

        let new_direction = get_next_direction(map.get(Point {x: new_x, y:new_y}), &direction.opposite_direction());
        if new_direction != Direction::Invalid {
            directions.push(*direction);
        }
    });

    if directions.len() != 2 {
        panic!("Map has invalid starting location as there are more than 2 valid directions");
    }

    (directions.remove(1), directions.remove(0))
}

fn get_next_direction(character: char, approaching_direction: &Direction) -> Direction {
    match approaching_direction {
        Direction::North => {
            match character {
                '|' => Direction::South,
                'L' => Direction::East,
                'J' => Direction::West,
                _ => Direction::Invalid
            }
        },
        Direction::East => {
            match character {
                '-' => Direction::West,
                'L' => Direction::North,
                'F' => Direction::South,
                _ => Direction::Invalid
            }
        },
        Direction::South => {
            match character {
                '|' => Direction::North,
                'F' => Direction::East,
                '7' => Direction::West,
                _ => Direction::Invalid
            }
        },
        Direction::West => {
            match character {
                '-' => Direction::East,
                '7' => Direction::South,
                'J' => Direction::North,
                _ => Direction::Invalid
            }
        },
        Direction::Invalid => {
            Direction::Invalid
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_loop_distance() {
        let contents = String::from(".....\n.S-7.\n.|.|.\n.L-J.\n.....");

        let map: Map = Map::new(contents);

        let distance = get_furthest_location(&map);
        assert_eq!(4, distance);
    }

    #[test]
    fn test_square_loop_starting_location() {
        let contents = String::from(".....\n.S-7.\n.|.|.\n.L-J.\n.....");
        
        let map: Map = Map::new(contents);

        assert_eq!(map.starting_location, Point::new(1, 1));
        let (direction_one, direction_two) = get_starting_directions(&map);

        if !(direction_one == Direction::East && direction_two == Direction::South || direction_one == Direction::South && direction_two == Direction::East) {
            panic!("The test has failed");
        }
    }

    #[test]
    fn direction_to_movement_tests() {
        assert_eq!(DIRECTION_TO_MOVEMENT[&Direction::North], Point::new(0, -1));
        assert_eq!(DIRECTION_TO_MOVEMENT[&Direction::East], Point::new(1, 0));
        assert_eq!(DIRECTION_TO_MOVEMENT[&Direction::South], Point::new(0, 1));
        assert_eq!(DIRECTION_TO_MOVEMENT[&Direction::West], Point::new(-1, 0));
    }

    #[test]
    fn character_and_direction_to_new_direction_tests() {
        assert_eq!(get_next_direction('|', &Direction::North), Direction::South);
        assert_eq!(get_next_direction('J', &Direction::West), Direction::North);
        assert_eq!(get_next_direction('F', &Direction::East), Direction::South);
        assert_eq!(get_next_direction('-', &Direction::West), Direction::East);
        assert_eq!(get_next_direction('7', &Direction::South), Direction::West);
        assert_eq!(get_next_direction('|', &Direction::South), Direction::North);
        assert_eq!(get_next_direction('L', &Direction::North), Direction::East);
        assert_eq!(get_next_direction('-', &Direction::East), Direction::West);
        assert_eq!(get_next_direction('J', &Direction::North), Direction::West);
        assert_eq!(get_next_direction('F', &Direction::South), Direction::East);
    }
}