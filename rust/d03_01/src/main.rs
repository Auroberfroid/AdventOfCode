use std::{env, fs, path::{Path, PathBuf}, vec};

#[derive(Debug)]
struct PartNumber {
    value: u32,
    top: String,
    bot: String,
    left: String,
    right: String,
    is_valid: bool
}

impl PartNumber {
    fn new(value: u32, top: String, bot: String, left: String, right: String) -> PartNumber {
        let mut is_valid = false;
        if  left.as_str() != "."  
        ||  right.as_str() != "." 
        ||  top.as_str() != ".".repeat(top.chars().count()).as_str()
        ||  bot.as_str() != ".".repeat(bot.chars().count()).as_str() {
            is_valid = true;
        }
        PartNumber {
            value,
            top,
            bot,
            left,
            right,
            is_valid: is_valid
        }
    }
}

fn get_input(filename: &str) -> Option<Vec<PartNumber>> {
    // Get full file path
    let mut filepath: PathBuf = PathBuf::new();
    match env::current_dir() {
        Ok(wd) => {
            filepath = wd;
        }
        Err(error) => {
            eprintln!("Error while getting the current working directory: {error}");
            ()
        }
    }
    let input_dir = "files";
    filepath = filepath.join(Path::new(input_dir)).join(Path::new(filename));

    // Read and process content
    // We will 'trick' the input by adding a fisrt and a last line full of '.' And full left and right columns full of '.'
    let mut result: Vec<PartNumber> = Vec::new(); 
    match fs::read_to_string(filepath) {
        Ok(content) => {
            // Get an iterator item containing each line and index
            let raw_lines_iter = content.split('\n');

            // Get the lenght of a line, which is static for the whole input file
            let len_line: usize;
            match raw_lines_iter.clone().next() {
                Some(next_line) => {
                    len_line = next_line.chars().count();
                }
                None => {
                    eprintln!("Couldn't get the first line from the iterator object...");
                    return None;
                }
            }

            // Add first and last line
            let fake_line_str = ".".repeat(len_line);
            let fake_first_line = vec![fake_line_str.as_str()].into_iter();
            let fake_last_line = fake_first_line.clone();
            let _lines_iter = fake_first_line.chain(raw_lines_iter).chain(fake_last_line);
            let mut vec_lines: Vec<String> = Vec::new();
            for line in _lines_iter {
                vec_lines.push(format!(".{}.", line));
            }
            // Create iterator from the vector to go through the lines
            let all_lines_iter = vec_lines.clone().into_iter();

            // We can now browse through the iterator without concerning about no char surrounding a number
            for (index_line, line) in all_lines_iter.enumerate() {
                let mut last_was_number = false;
                let mut number:u32 = 0;
                for (index_huchar, huchar) in line.chars().enumerate() {
                    match huchar.to_digit(10) {
                        Some(val) => {
                            last_was_number = true;
                            number = 10 * number +  val;
                        }
                        None => {
                            if last_was_number {
                                // Get number len to add its top and diag chars
                                let mut _n:f32 = number as f32;
                                let mut number_len = 1;
                                while _n > 10f32 {
                                    _n = _n / 10f32;
                                    number_len += 1;
                                }
                                let mut left = String::new();
                                match vec_lines[index_line].chars().nth(index_huchar-(number_len+1)) {
                                    Some(val) => {
                                        left.push(val);
                                    }
                                    None => {
                                        eprintln!("Error while getting the left character: {} of line: {}", index_huchar - 1, index_line);
                                        return None;
                                    }
                                }
                                let mut right = String::new();
                                match vec_lines[index_line].chars().nth(index_huchar) {
                                    Some(val) => {
                                        right.push(val);
                                    }
                                    None => {
                                        eprintln!("Error while getting the right character: {} of line: {}", index_huchar + 1, index_line);
                                        return None;
                                    }
                                }
                                let mut top = String::new();
                                top.push_str(&vec_lines[index_line-1][index_huchar-number_len-1..=index_huchar]);
                                let mut bot = String::new();
                                bot.push_str(&vec_lines[index_line+1][index_huchar-number_len-1..=index_huchar]);
                                
                                let pn = PartNumber::new(number, top, bot, left, right);
                                result.push(pn);
                                number = 0;
                                last_was_number = false;
                            }
                        }
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error while getting lines: {error}");
            return None;
        }
    }
    Some(result)
}

fn main() {
    let filename = "input.txt";
    let pn_vec: Vec<PartNumber>;
    match get_input(filename) {
        Some(val) => {
            pn_vec = val;
        }
        None => {
            pn_vec = Vec::new();
        }
    }
    let mut result: u32 = 0;
    for pn in pn_vec {
        if pn.is_valid {
            result += pn.value;
        }
    }
    println!("Result: {result}");
}