use std::{fs, collections::HashMap};

struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should be able to read the input");

    let mut iter = contents.lines();
    let instructions = iter.next().expect("Instructions should exist");
    iter.next(); // skip empty line

    // parse map
    let map: HashMap::<String, Node> = HashMap::new();
    let (curr_node, map) = parse_map(map, iter);

    let vec: Vec<char> = instructions.chars().collect();
    let steps = get_total_steps(String::from("AAA"), vec, map, &"ZZZ");

    println!("Total steps: {}", steps);
}

fn parse_map<'a, T> (mut map: HashMap<String, Node>, mut iter: T) -> (String, HashMap<String, Node>) 
where T: Iterator<Item = &'a str>
{
    let root_node_str = iter.next().expect("Root node");
    
    let node = get_node(root_node_str);

    let curr_node = node.id.clone();
    
    map.insert(node.id.clone(), node);

    for line in iter {
        let node = get_node(line);
        map.insert(node.id.clone(), node);
    }

    (curr_node, map)
}

fn get_total_steps(mut curr_node: String, vec: Vec<char>, map: HashMap<String, Node>, target: &str) -> i32 {
    let mut i = 0;
    let mut steps = 0;
    loop {
        if curr_node == target {
            break;
        }

        let direction = vec[i];

        let node = map.get(&curr_node).expect("Should have a node");

        dbg!(steps);
        if  direction == 'L' {
            curr_node = node.left.clone();
        }
        else {
            curr_node = node.right.clone();
        }
        i = (i + 1) % vec.len();
        steps += 1;
    }
    steps
}

fn get_node(node_str: &str) -> Node {
    let mut iter = node_str.split('=');
    let id = iter.next().expect("node id should exist").trim().to_string();
    
    let mut iter = iter.next()
        .expect("Left and Right node values should exist")
        .split(',');

    let left = iter.next().expect("Left node should exist").trim().replace('(', "");
    let right = iter.next().expect("Right node should exist").trim().replace(')', "");
    
    Node {
        id,
        left,
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn parse_map_works() {
        let input = String::from("AAA = (BBB, CCC)\nBBB = (AAA, AAA)\nCCC = (DDD, EEE)\nDDD = (CCC, AAA)\nEEE = (AAA, AAA)");
        let map: HashMap<String, Node> = HashMap::new();
        let (_, map) = parse_map(map, input.split('\n'));

        // manually create equivalent hashmap:
        let mut expected_map: HashMap<String, Node> = HashMap::new();
        add_to_map(&mut expected_map, "AAA", "BBB", "CCC");
        add_to_map(&mut expected_map, "BBB", "AAA", "AAA");
        add_to_map(&mut expected_map, "CCC", "DDD", "EEE");
        add_to_map(&mut expected_map, "DDD", "CCC", "AAA");
        add_to_map(&mut expected_map, "EEE", "AAA", "AAA");

        assert_maps_are_equal(&expected_map, &map);
    }

    #[test]
    fn traverse_map_from_instructions_works() {
        let mut map: HashMap<String, Node> = HashMap::new();
        add_to_map(&mut map, "AAA", "BBB", "CCC");
        add_to_map(&mut map, "BBB", "AAA", "AAA");
        add_to_map(&mut map, "CCC", "DDD", "EEE");
        add_to_map(&mut map, "DDD", "CCC", "AAA");
        add_to_map(&mut map, "EEE", "AAA", "AAA");

        let total_steps = get_total_steps(String::from("AAA"), vec!['L','R','R','R'], map, &"EEE");

        assert_eq!(4, total_steps);
    }

    #[test]
    fn traverse_map_from_instructions_works_overflow() {
        let mut map: HashMap<String, Node> = HashMap::new();
        add_to_map(&mut map, "AAA", "BBB", "CCC");
        add_to_map(&mut map, "BBB", "EEE", "CCC");
        add_to_map(&mut map, "CCC", "DDD", "EEE");
        add_to_map(&mut map, "DDD", "CCC", "BBB");
        add_to_map(&mut map, "EEE", "AAA", "AAA");

        let total_steps = get_total_steps(String::from("AAA"), vec!['L','R'], map, &"EEE");

        assert_eq!(5, total_steps);
    }

    #[test]
    fn traverse_map_from_instructions_works_odd_overflow() {
        let mut map: HashMap<String, Node> = HashMap::new();
        add_to_map(&mut map, "AAA", "BBB", "CCC");
        add_to_map(&mut map, "BBB", "EEE", "CCC");
        add_to_map(&mut map, "CCC", "DDD", "EEE");
        add_to_map(&mut map, "DDD", "CCC", "BBB");
        add_to_map(&mut map, "EEE", "AAA", "AAA");

        let total_steps = get_total_steps(String::from("AAA"), vec!['L','R','L'], map, &"EEE");

        assert_eq!(5, total_steps);
    }


    fn add_to_map(expected_map: &mut HashMap<String, Node>, id: &str, left: &str, right: &str) {
        expected_map.insert(String::from(id),
            Node {
                id: String::from(id),
                left: String::from(left),
                right: String::from(right),
            });
    }

    fn assert_maps_are_equal(expected_map: &HashMap<String, Node>, actual_map: &HashMap<String, Node>) {
        if (expected_map.len() != actual_map.len())
        {
            panic!("There is a different number of keys in each map");
        }

        for key in expected_map.keys() {
            let actual_node = actual_map.get(key).expect("Should Exist if maps are equal");
            let expected_node = expected_map.get(key).expect("Should Exist by definition");

            if actual_node.id != expected_node.id 
            {
                panic!("Node ids not equivalent");
            }

            if actual_node.left != expected_node.left {
                panic!("Node left vals not equivalent")
            }

            if actual_node.right != expected_node.right {
                panic!("Node right vals not equivalent")
            } 
        }
    }
}
