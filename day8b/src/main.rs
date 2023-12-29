use petgraph::{graph::{NodeIndex, Graph}, visit::EdgeRef};
use std::{fs::{self, File}, collections::HashMap, io::Write};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file"); 

    let mut lines = contents.lines();
    let instructions = lines.next().expect("Instructions should exist");
    lines.next(); // skip blank line

    let mut g: Graph<String, String> = Graph::new();
    
    let mut map: HashMap<String, NodeIndex> = HashMap::new();

    for line in lines {
        let mut split = line.split("=");
        let node_val = split.next().expect("Should have a left side").trim().to_string();
        let edges_to = split.next().expect("Should have a right side").trim().to_string();
        let mut edges_split = edges_to.split(", ");

        let a = get_or_add_node(&mut g, &mut map, node_val); 

        let left_val = edges_split.next().expect("Should have a value").trim().replace('(', "");
        let right_val = edges_split.next().expect("Should have a value").trim().replace(')', "");

        let left = get_or_add_node(&mut g, &mut map, left_val); 
        let right = get_or_add_node(&mut g, &mut map, right_val); 
        g.add_edge(a, left, String::from("L"));
        g.add_edge(a, right, String::from("R"));

    }

    write_to_file_graph(&g).expect("Should have been able to write to file");

    let starting_points = get_starting_points(&g, &|x| (g.node_weight(x).expect("Should have a weight").contains('A')));

    let answers : Vec<u64> = starting_points.iter()
        .map(|x: &NodeIndex| get_cycle_length(&g, *x, &instructions, 100000) as u64)
        .collect();

    println!("Answer: {}", calculate_lcm(answers));
}

fn calculate_lcm(values: Vec<u64>) -> u64 {
    let mut lcm = values[0];
    for i in 1..values.len() {
        lcm = (lcm * values[i]) / gcd(lcm, values[i]);
    }
    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn get_or_add_node(g: &mut Graph<String, String>, map: &mut HashMap<String, NodeIndex>, val: String) -> NodeIndex {
    if map.contains_key(&val) {
        return *map.get(&val).expect("Should have a value");
    }

    let node = g.add_node(val.clone());
    map.insert(val, node);
    node
}

fn get_starting_points(g: &Graph<String, String>, f: &dyn Fn(NodeIndex) -> bool) -> Vec<NodeIndex> {
    let mut starting_points = Vec::new();
    for node in g.node_indices() {
        if f(node) {
            starting_points.push(node);
        }
    }
    starting_points
} 

fn get_cycle_length(g: &Graph<String, String>, starting_point: NodeIndex, instructions: &str, max: usize) -> u32 {

    let mut current_node = starting_point;
    let instructions: Vec<char> = instructions.chars().collect();
    let mut index: usize = 0;
    
    let mut answers : Vec<u32> = Vec::new();
    while index < max {
        let node_val = g.node_weight(current_node).expect("Should have a weight");
        if node_val.contains('Z') {
            let steps: usize = index + 1;
            println!("Found Z for starting point {} at step {}", g.node_weight(starting_point).expect("Should have a weight"), steps);
            answers.push(steps as u32);
        }

        let edges = g.edges(current_node);
        let instruction = instructions[index % instructions.len()];
        for edge in edges {
            let weight = edge.weight();
            if weight.contains(instruction) {
                current_node = edge.target();
                break;
            }
        }

        index += 1;
    }
    
    if answers.len() < 3 {
        panic!("Not enough answers to calculate differences, please increase max.")
    }

    let differences = answers.windows(2).map(|x| x[1] - x[0]).collect::<Vec<u32>>();
    let all_equal = differences.windows(2).all(|x| x[0] == x[1]);

    if !all_equal {
        panic!("Not all differences are equal - cannot use LCM algorithm")
    }

    return differences[0];
}

fn write_to_file_graph(g: &Graph<String, String>) -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    writeln!(file, "digraph G {{")?;
    for edge in g.edge_indices() {
        let (a, b) = g.edge_endpoints(edge).expect("Should have endpoints");
        writeln!(file, "    {} -> {}", g.node_weight(a).expect("Should have a weight"), g.node_weight(b).expect("Should have a weight"))?;
    }
    writeln!(file, "}}")?;

    Ok(())
}