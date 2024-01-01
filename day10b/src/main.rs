use std::{collections::{HashMap, HashSet, VecDeque}, fs, ops::Add};

use lazy_static::lazy_static;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let map: Map = Map::new(contents);

    let answer = get_area_of_inside(&map);

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

#[derive(PartialEq, Eq, Hash, Debug)]
enum Turn {
    Left,
    Right,
    NoTurn
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

fn get_area_of_inside(map: &Map) -> u32 {
    // Travel in both direction until you have visited all the nodes
    // find starting directions
    let (first_direction, second_direction) = get_starting_directions(map);
    let starting_character = convert_connections_to_character(&first_direction, &second_direction);

    let first_pointer = map.starting_location + &DIRECTION_TO_MOVEMENT[&first_direction];

    let (boundary, turns) = calc_boundary_and_get_turns(map, &first_pointer, &first_direction);

    println!("Left turns: {}, Right Turns: {}", turns.get(&Turn::Left).expect("Should have a value"), turns.get(&Turn::Right).expect("Should have a value"));

    let clockwise = turns[&Turn::Left] < turns[&Turn::Right];

    println!("Is clockwise: {}", clockwise);

    // We now traverse counter-clockwise, and look to the left on every turn
    let mut direction = if clockwise { first_direction.opposite_direction() } else { second_direction.opposite_direction() };
    let mut pointer = map.starting_location; 
     
    let mut area : HashSet<Point> = HashSet::new();
    let mut traversed : HashSet<Point> = HashSet::new();
    traversed.insert(map.starting_location.clone());

    loop {
        let mut c_at_location = map.get(pointer);

        if c_at_location == 'S' {
            c_at_location = starting_character;
        }
        
        let points_to_left = get_points_to_left(c_at_location, &direction.opposite_direction());
        for point in points_to_left {
            if !area.contains(&(point + &pointer)) {
                let fill = flood_fill(&(point + &pointer), &boundary);
                area.extend(&fill);  
            }
        }

        direction = get_next_direction(c_at_location, &direction.opposite_direction());

        if direction == Direction::Invalid { panic!("New first direction is invalid"); }

        pointer = pointer + &DIRECTION_TO_MOVEMENT[&direction];

        if traversed.contains(&pointer) {
            break;
        }

        traversed.insert(pointer.clone());
    }

    area.len() as u32
}

fn flood_fill(starting_point: &Point, boundary: &HashSet<Point>) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(starting_point.clone());
    while queue.len() > 0 {
        let curr_point = queue.pop_front().expect("Should be a point still");
        if boundary.contains(&curr_point) {
            continue;
        }

        if !visited.contains(&curr_point) {
            visited.insert(curr_point.clone());

            queue.push_back(curr_point.clone() + &DOWN);
            queue.push_back(curr_point.clone() + &UP);
            queue.push_back(curr_point.clone() + &RIGHT);
            queue.push_back(curr_point.clone() + &LEFT);
        }
    }

    visited
}

fn convert_connections_to_character(direction_one: &Direction, direction_two: &Direction) -> char {
    match (direction_one, direction_two)
    {
        (Direction::South, Direction::East) => 'F',
        (Direction::South, Direction::North) => '|',
        (Direction::South, Direction::West) => '7',
        (Direction::North, Direction::East) => 'L',
        (Direction::North, Direction::South) => '|',
        (Direction::North, Direction::West) => 'J',
        (Direction::East, Direction::North) => 'L',
        (Direction::East, Direction::South) => 'F',
        (Direction::East, Direction::West) => '-',
        (Direction::West, Direction::North) => 'J',
        (Direction::West, Direction::East) => '-',
        (Direction::West, Direction::South) => '7',
        (_, _) => panic!("Invalid option")
    }
}

fn calc_boundary_and_get_turns(map: &Map, first_point: &Point, first_direction: &Direction) -> (HashSet<Point>, HashMap<Turn, u32>) {
    let mut boundary : HashSet<Point> = HashSet::new();
    let mut turns : HashMap<Turn, u32> = HashMap::new();
    turns.insert(Turn::Right, 0);
    turns.insert(Turn::Left, 0);
    turns.insert(Turn::NoTurn, 0);

    let mut first_pointer = first_point.to_owned();
    let mut first_direction = first_direction.to_owned();

    boundary.insert(map.starting_location.clone());
    boundary.insert(first_pointer.clone());

    loop {
        let first_char = map.get(first_pointer);
        let new_direction = get_next_direction(first_char, &first_direction.opposite_direction());
        let turn = direction_to_turn(&first_direction, &new_direction);
        *turns.entry(turn).or_default() += 1;
        first_direction = new_direction;

        if first_direction == Direction::Invalid {
            panic!("New first direction is invalid");
        }

        first_pointer = first_pointer + &DIRECTION_TO_MOVEMENT[&first_direction];

        if boundary.contains(&first_pointer) {
            break;
        }

        boundary.insert(first_pointer.clone());
    }
    (boundary, turns)
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

fn direction_to_turn(starting_direction: &Direction, resulting_direction: &Direction) -> Turn {
    match starting_direction {
        Direction::North => {
            match resulting_direction {
                Direction::North => Turn::NoTurn,
                Direction::East => Turn::Right,
                Direction::West => Turn::Left,
                _ => panic!("No piece supports turning around")
            }
        },
        Direction::East => {
            match resulting_direction {
                Direction::East => Turn::NoTurn,
                Direction::South => Turn::Right,
                Direction::North => Turn::Left,
                _ => panic!("No piece supports turning around")   
            }
        },
        Direction::South => {
            match resulting_direction {
                Direction::South => Turn::NoTurn,
                Direction::West => Turn::Right,
                Direction::East => Turn::Left,
                _ => panic!("No piece supports turning around")
            }
        },
        Direction::West => {
            match resulting_direction {
                Direction::West => Turn::NoTurn,
                Direction::North => Turn::Right,
                Direction::South => Turn::Left,
                _ => panic!("No piece supports turning around")
            }
        },
        Direction::Invalid => panic!("No direction for Invalid direction")
    }
}

const DOWN: Point = Point{ x: 0, y: 1};
const DOWN_RIGHT: Point = Point{ x: 1, y: 1};
const DOWN_LEFT: Point = Point{ x: -1, y: 1};
const LEFT: Point = Point{ x: -1, y: 0 };
const RIGHT: Point = Point{ x: 1, y: 0 };
const UP: Point = Point{ x: 0, y: -1};
const UP_LEFT: Point = Point{ x: -1, y: -1};
const UP_RIGHT: Point = Point{ x: 1, y: -1};

fn get_points_to_left(character: char, approaching_direction: &Direction) -> Vec<Point> {
    match (character, approaching_direction) {
        ('|', Direction::North) => vec![RIGHT],
        ('|', Direction::South) => vec![LEFT],
        ('L', Direction::North) => vec![UP_RIGHT],
        ('L', Direction::East) => vec![DOWN, DOWN_LEFT, LEFT],
        ('-', Direction::East) => vec![DOWN],
        ('-', Direction::West) => vec![UP],
        ('J', Direction::North) => vec![RIGHT, DOWN_RIGHT, DOWN],
        ('J', Direction::West) => vec![UP_LEFT],
        ('7', Direction::West) => vec![UP, UP_RIGHT, RIGHT],
        ('7', Direction::South) => vec![DOWN_LEFT],
        ('F', Direction::South) => vec![LEFT, UP_LEFT, UP],
        ('F', Direction::East) => vec![DOWN_RIGHT],
        (c, d) => panic!("Unknown combination: {}, {:?}", c, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_loop_area() {
        let contents = String::from(".....\n.S-7.\n.|.|.\n.L-J.\n.....");

        let map: Map = Map::new(contents);

        let area = get_area_of_inside(&map);
        assert_eq!(1, area);
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

    #[test]
    fn test_left_turns() {
        assert_eq!(direction_to_turn(&Direction::East, &Direction::North), Turn::Left);
        assert_eq!(direction_to_turn(&Direction::North, &Direction::West), Turn::Left);
        assert_eq!(direction_to_turn(&Direction::West, &Direction::South), Turn::Left);
        assert_eq!(direction_to_turn(&Direction::South, &Direction::East), Turn::Left);   
    }

    #[test]
    fn test_no_turns() {
        assert_eq!(direction_to_turn(&Direction::North, &Direction::North), Turn::NoTurn);
        assert_eq!(direction_to_turn(&Direction::West, &Direction::West), Turn::NoTurn);
        assert_eq!(direction_to_turn(&Direction::South, &Direction::South), Turn::NoTurn);
        assert_eq!(direction_to_turn(&Direction::East, &Direction::East), Turn::NoTurn);
    }

    #[test]
    fn test_right_turns() {
        assert_eq!(direction_to_turn(&Direction::North, &Direction::East), Turn::Right);
        assert_eq!(direction_to_turn(&Direction::West, &Direction::North), Turn::Right);
        assert_eq!(direction_to_turn(&Direction::South, &Direction::West), Turn::Right);
        assert_eq!(direction_to_turn(&Direction::East, &Direction::South), Turn::Right);
    }
}