use std::{env::current_dir, fmt::{Formatter, Result as FmtResult, Display}, fs::read_to_string, path::PathBuf};

const HASH_TABLE_SIZE: usize = 678; 

fn debug_print<T: Display>(arg: T) {
    println!("ARO DEBUG: {arg}");
}

enum Direction {
    Right,
    Left,
    Unknown
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
            Direction::Unknown => write!(f, "Unknown")
        }
    }
}


struct Node {
    id: [char; 3],
    left: [char; 3],
    right: [char; 3]
}

impl Node {
    fn get_as_str(&self, attrib: &str) -> String {
        let mut res: String = String::new();
        match attrib {
            "id" => {
                for c in self.id {
                    res.push(c);
                }
            }
            "left" => {
                for c in self.left {
                    res.push(c);
                }
            }
            "right" => {
                for c in self.right {
                    res.push(c);
                }
            }
            _ => {
                eprintln!("Invalid attrib: {}", attrib);
                res.push('N');
                res.push('O');
            }
        }
        res
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Node {} => ({}, {})", self.get_as_str("id"), self.get_as_str("left"), self.get_as_str("right"))
    }
}

struct BoostedString(String);

impl Node {
    fn new(id: [char; 3], left: [char; 3], right: [char; 3]) -> Self {
        Self {
            id,
            left,
            right
        }
    }
}

impl Into<[char; 3]> for BoostedString {
    fn into(self) -> [char; 3] { 
        let mut res: [char; 3] = ['.'; 3];
        for (i, iter_char) in self.0.chars().enumerate() {
            res[i] = iter_char;
            if i == 2 {break}
        }
        res
    }
}

fn id_converter(x: &str) -> [char; 3] {
    let mut res: [char; 3] = ['.'; 3];
        for (i, iter_char) in x.chars().enumerate() {
            res[i] = iter_char;
            if i == 2 {break}
        }
        res
}

fn get_input(filename: &str) -> Result<(Vec<Node>, Vec<Direction>), String> {
    let input_dir = "files";
    let filepath: PathBuf;

    match current_dir() {
        Ok(val) => {
            filepath = val.join(input_dir).join(filename);
        }
        Err(error) => {
            let err_msg = format!("[Error while getting the current directory. Trace: {error}]");
            return Err(err_msg);
        }
    }

    let content: String;
    match read_to_string(filepath) {
        Ok(val) => {
            content = val;
        }
        Err(error) => {
            let err_msg = format!("[Error while reading the file content. Trace: {error}]");
            return Err(err_msg);
        }

    }

    let mut directions = Vec::<Direction>::new();
    let mut nodes = Vec::<Node>::new();
    for (index_line, line) in content.split('\n').enumerate() {
        if index_line == 0 {
            for direction_char in line.chars() {
                match direction_char {
                    'R' => {
                        directions.push(Direction::Right);
                    }
                    'L' => {
                        directions.push(Direction::Left);
                    }
                    _ => {
                        let err_msg = format!("[Error while processing the direction. Trace: Unknown direction character parsed]");
                        return Err(err_msg);
                    }
                }
            }
            continue;
        }
        if line != "" {
            let equal_split = line.split('=').map(|x| x.to_owned()).collect::<Vec<String>>();
            let id: [char; 3] = id_converter(&equal_split[0]);
            
            
            let coma_split = equal_split[1].split(',').map(|x| x.to_owned()).collect::<Vec<String>>();
            // debug_print(&coma_split[0]);
            let left: [char; 3] = id_converter(coma_split[0].replace("(", "").trim());
            let right: [char; 3] = id_converter(coma_split[1].replace(")", "").trim());
            nodes.push(Node::new(id, left, right));
        }
    }
    Ok((nodes, directions))
}

fn main() -> Result<(), String>{
    let filename: &str = "input.txt";

    let mut nodes: Vec<Node>;
    let mut directions: Vec<Direction>;
    match get_input(filename) {
        Ok(val) => {
            nodes = val.0;
            directions = val.1;
        }
        Err(error) => {
            let err_msg = format!("[Error while getting the input: {error}]");
            eprintln!("{err_msg}");
            return Err(err_msg);
        }
    }

    for direct in directions {
        print!("{direct} ");
    }
    println!("\n######################");
    for node in nodes {
        println!("{node}");
    }

    Ok(())
}
