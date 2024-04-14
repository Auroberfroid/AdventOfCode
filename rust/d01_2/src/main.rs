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

fn get_first_digit_in_line(line: &String, number_list: &[Number; 10], is_reversed: bool) -> Option<usize> {
    // Contains a Number and a buffer that will keep track of the matching chars for each number
    let mut number_lettercheckbuff_array: [String; 10] = Default::default();

    // Loop through the line to check if matched with numbers
    for line_character in line.chars() {
        // Loop through numbers to match the numeric of the letter
        for index_num in 0..number_list.len() {
            if line_character == number_list[index_num].get_numeric_as_char() {
                return Some(index_num); // Return directly if match a numeric 
            }
            else { // If not equal to a numeric check if equal to the letters, if yes then keep track of it in the buffer 'number_lettercheckbuff_array'
                // Get the letter_char index depending on the current buffer size of the current number
                match number_list[index_num].get_letter_char_from_index(number_lettercheckbuff_array[index_num].len(), is_reversed) {
                    Some(letter_char) => { // means that the [index] char of the letter from the current number exists
                        // If they are equal then push it onto the buffer
                        if line_character == letter_char {
                            number_lettercheckbuff_array[index_num].push(letter_char);
                        }
                        // Also check if the len within the buffer matches the len of the number's letter, if yes then its a letter match !
                        if number_lettercheckbuff_array[index_num].len() == number_list[index_num].get_letters(is_reversed).len() {
                            return Some(index_num); // Return directly if match a letter
                        }
                    }
                    None => {}
                }
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
    letters: String
}

impl Number {
    fn get_letters(&self, is_reversed: bool) -> String {
        let result: String;
        if is_reversed {
            result =  get_reversed_string(&self.letters);
        }
        else {
            result = self.letters.clone();
        }
        result
    }

    fn get_numeric_as_char(&self) -> char {
        (self.numeric + 0x30) as char
    }

    fn get_letter_char_from_index(&self, index: usize, is_reversed: bool) -> Option<char> {
        if index < self.get_letters(is_reversed).len() {
            Some(self.get_letters(is_reversed).as_bytes()[index] as char)
        }
        else {
            None
        }
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
    let numbers: [Number; 10] = [Number {numeric: 0, letters: String::from("zero")},
                                               Number {numeric: 1, letters: String::from("one")},
                                               Number {numeric: 2, letters: String::from("two")},
                                               Number {numeric: 3, letters: String::from("three")},
                                               Number {numeric: 4, letters: String::from("four")},
                                               Number {numeric: 5, letters: String::from("five")},
                                               Number {numeric: 6, letters: String::from("six")},
                                               Number {numeric: 7, letters: String::from("seven")},
                                               Number {numeric: 8, letters: String::from("eight")},
                                               Number {numeric: 9, letters: String::from("nine")}];

    let mut result: u64 = 0;
    // Process lines
    for line in input_lines {
        // Process first digit
        let left_digit: u8;
        match get_first_digit_in_line(&line, &numbers, false) {
            Some(index) => {
                match index.try_into() {
                    Ok(val) => {
                        left_digit = val;
                        println!("ARO DEBUG: left: {}", left_digit);
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
        match get_first_digit_in_line(&get_reversed_string(&line), &numbers, true) {
            Some(index) => {
                match index.try_into() {
                    Ok(val) => {
                        right_digit = val;
                        println!("ARO DEBUG: right: {}", right_digit);
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