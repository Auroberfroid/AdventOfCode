use std::{env::current_dir, fs::read_to_string, path::PathBuf, collections::HashMap};
use num::integer::lcm;
const DEBUG: bool = false;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String
}

impl Node {
    fn new(id: &str, left: &str, right: &str) -> Self {
        Self {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string()
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left
}


/// Parse the input of the AOC D8 into a HashMap of Nodes and a Vec of Directions
fn get_input(filename: &str) -> Result<(HashMap<String, Node>, Vec<Direction>), String> {
    
    // Returned processed input
    let mut directions = Vec::<Direction>::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();


    // Open the file
    let filepath: PathBuf;
    let input_dir = "files";
    match current_dir() {
        Ok(val) => {
            filepath = val.join(input_dir).join(filename);
        }
        Err(error) => {
            let err_msg = format!("[Error while getting the current directory: {error}]");
            return Err(err_msg);
        }
    }

    // Get the content 
    let content: String;
    match read_to_string(filepath) {
        Ok(val) => {
            content = val;
        }
        Err(error) => {
            let err_msg = format!("[Error while getting the file content: {error}]");
            return Err(err_msg); 
        }
    }

    // Iterate over lines 
    for (line_index, line) in content.lines().enumerate() {
        // Process the directions
        if line_index == 0 {
            for dir in line.chars() {
                match dir {
                    'L' => {
                        directions.push(Direction::Left);
                    }
                    'R' => {
                        directions.push(Direction::Right);
                    }
                    _ => {
                        let err_msg = "Error while getting the file content".to_string();
                        return Err(err_msg);
                    }
                }
            }
        }
        // Skip the line 1 and process the other as Nodes
        // AAA = (BBB, CCC)
        else if line_index > 1 {
            let mut id: &str = "000";
            let mut left: &str = "000";
            let mut right: &str = "000";
            for (eq_split_index, eq_split) in line.split('=').enumerate() {
                if eq_split_index == 0 {
                    id = eq_split.trim();
                }
                else {
                    for (comma_eq_split_index, comma_eq_split) in eq_split.split(',').enumerate() {
                        if comma_eq_split_index == 0 {
                            left = comma_eq_split.trim().trim_matches('(').trim();
                        }
                        else {
                            right = comma_eq_split.trim().trim_matches(')').trim();
                        }
                    }
                }
            }
            // Insert nodes into the HashMap
            let node = Node::new(id, left, right);
            nodes.insert(node.id.clone(), node);
        }
    }

    Ok((nodes, directions))
}


/// Compute the required amount of steps to get to the ending node from the starting node
fn compute_steps(starting_node_id: &str, node_hashmap: &HashMap<String, Node>, directions: &Vec<Direction>) -> usize {
    let mut count: usize = 0;
    let mut current_node = node_hashmap.get(starting_node_id).unwrap();

    loop {
        for dir in directions {
            match dir {
                &Direction::Left => {
                    current_node = node_hashmap.get(&current_node.left).unwrap();
                }
                &Direction::Right => {
                    current_node = node_hashmap.get(&current_node.right).unwrap();
                }
            }
            count += 1;
            if current_node.id.ends_with('Z') {
                return count;
            }
        }
    }
}

fn main() -> Result<(), i8> {
    let filename: &str;
    if DEBUG {
        filename = "input_debug.txt";
    }
    else {
        filename = "input.txt";
    }

    // Parse the input to get map and direction
    let directions: Vec<Direction>;
    let nodes: HashMap<String, Node>;
    match get_input(filename) {
        Ok((val_nodes, val_directions)) => {
            nodes = val_nodes;
            directions = val_directions;
            println!("Get input success");

        }
        Err(error) => {
            let err_msg = format!("[Error while get_input. Error: {error}]");
            eprintln!("{}", err_msg);
            return Err(-1);
        }
    }

    // Get nodes that ends with an A
    let mut starting_nodes = Vec::<Node>::new();
    for node in nodes.clone().into_iter() {
        if node.0.ends_with('A') {
            starting_nodes.push(node.1);
        }
    }

    // Compute steps for each starting node
    let mut res: usize = 1;
    for starting_node in starting_nodes {
        res = lcm::<usize>(res, compute_steps(&starting_node.id, &nodes, &directions));
    }

    println!("The result is {}", res);

    Ok(())
}
