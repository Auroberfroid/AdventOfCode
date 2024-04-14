use std::{env, fs, path::{Path, PathBuf}};

/// Return the input file as list of lines
fn get_input(filepath: &Path) -> Vec<String> {
    let mut vec_lines: Vec<String> = Vec::new(); 
    match fs::read_to_string(filepath) {
        Ok(content) => {
            for line in content.split('\n') {
                vec_lines.push(line.to_owned());
            }
        }
        Err(error) => {
            eprintln!("Error while opening the file: {error}");
        }
    }
    vec_lines
}

fn get_first_digit_in_line(line: &String, number_list: &[Number; 10], number_list_size: usize) -> Option<usize> {
    for character in line.chars() {
        for i in 0..number_list_size {
            /* println!("ARO DEBUG: number[i] as char: {:?}", number_list[i]); */
            if character == number_list[i].get_numeric_as_char() {
                return Some(i);
            }
        }
    }
    None
}

fn get_reversed_string(input: &String) -> String {
    let mut result: String = String::from("");
    for character in input.chars().rev() {
        result.push(character);
    }
    result
}

#[derive(Debug)]
struct Number {
    numeric: u8,
}

impl Number {
    fn get_numeric_as_char(&self) -> char {
        (self.numeric + 0x30) as char
    }
}

fn main() {
    // Get the input data
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
    let input_filename = "input.txt";
    filepath = filepath.join(Path::new(input_dir)).join(Path::new(input_filename));
    let input_lines = get_input(&(filepath.as_path()));
    
    // Get list of numbers
    const NUMBER_LIST_SIZE: usize = 10;
    let numbers: [Number; NUMBER_LIST_SIZE] = [Number {numeric: 0},
                                               Number {numeric: 1},
                                               Number {numeric: 2},
                                               Number {numeric: 3},
                                               Number {numeric: 4},
                                               Number {numeric: 5},
                                               Number {numeric: 6},
                                               Number {numeric: 7},
                                               Number {numeric: 8},
                                               Number {numeric: 9}];

    let mut result: u64 = 0;

    // Process lines
    for line in input_lines {
        // Process first digit
        let left_digit: u8;
        match get_first_digit_in_line(&line, &numbers, NUMBER_LIST_SIZE) {
            Some(index) => {
                match index.try_into() {
                    Ok(val) => {
                        left_digit = val;
                    }
                    Err(error) => {
                        eprintln!("Error while converting usize into u8: {error}");
                        left_digit = 0;
                    }
                }
            }
            None => {
                eprintln!("No number found as first in the line: {line}");
                left_digit = 0;
            }
        }
        
        // Process last digit
        let right_digit: u8;
        match get_first_digit_in_line(&get_reversed_string(&line), &numbers, NUMBER_LIST_SIZE) {
            Some(index) => {
                match index.try_into() {
                    Ok(val) => {
                        right_digit = val;
                    }
                    Err(error) => {
                        eprintln!("Error while converting usize into u8: {error}");
                        right_digit = 0;
                    }
                }
            }
            None => {
                eprintln!("No number found as last in the line: {line}");
                right_digit = 0;
            }
        }

        result += u64::from(10 * left_digit + right_digit)
    }
    println!("RESULT is: {result}");
}