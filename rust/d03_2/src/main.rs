use std::{env, fmt, fs, path::{Path, PathBuf}, vec};

#[derive(Debug)]
struct InclusiveRange {
    start: usize,
    end: usize
}

impl InclusiveRange {
    fn range_insersect(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

struct Surface {
    x: usize,
    y: usize,
    widht: usize,
    height: usize
}

impl Surface {
    fn new(x: usize, y: usize, widht: usize, height: usize) -> Surface {
        Surface {
            x,
            y,
            widht,
            height
        }
    }
    ///  Returns a range for the x axis of the Surface
    fn get_x_range(&self) -> InclusiveRange {
        InclusiveRange {start: self.x, end: self.x + self.widht}
    }
    ///  Returns a range for the y axis of the Surface
    fn get_y_range(&self) -> InclusiveRange {
        InclusiveRange {start: self.y, end: self.y + self.height}
    }
    /// Returns true if self and other intersects, false otherwise
    fn surface_intersect(&self, other: &Self) -> bool {
        if self.get_x_range().range_insersect(&other.get_x_range())
        && self.get_y_range().range_insersect(&other.get_y_range()) {
            return true;
        }
        false
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Surface: x_range:{:?} | y_range:{:?}", self.get_x_range(), self.get_y_range())
    }
}

trait HasCoordinates {
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
    fn get_widht(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_surface(&self) -> Surface;
}



/// # Represents all number from the input
/// * The char_x and char_y coordinates are pointing from top to bot and left to right
/// * char_x and char_y are coordinates of the first numeric char
/// * is_valid set to true if at least one special char in its surrounding, else false
#[derive(Debug, Clone)]
struct PartNumber {
    value: u32,
    nb_len: usize,
    char_x: usize,
    char_y: usize
}

impl PartNumber {
    fn new(value: u32, nb_len: usize, char_x: usize, char_y: usize) -> PartNumber {
        PartNumber {
            value,
            nb_len,
            char_x,
            char_y,
        }
    }
}

impl HasCoordinates for PartNumber {
    /// char_x: x coord of the char
    fn get_x(&self) -> usize {
        self.char_x
    }
    /// char_y: y coord of the char
    fn get_y(&self) -> usize {
        self.char_y
    }
    /// PartNumber has a fixed height of 1
    fn get_height(&self) -> usize {
        0
    }
    /// Lenght of the number
    fn get_widht(&self) -> usize {
        self.nb_len - 1
    }
    /// Returns a Surface that defines the PartNumber area, can then be used to check if intersection between 2 Surfaces
    fn get_surface(&self) -> Surface {
        Surface::new(self.get_x(), self.get_y(), self.get_widht(), self.get_height())
    }
}

impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PartNumber value:{}, char_x:{}, char_y:{}, Surface:{}", self.value, self.char_x, self.char_y, self.get_surface())
    }
}

/// # Represents a Gear
/// * x and y are coordinates of the gear (on the bot left)
#[derive(Debug, Clone)]
struct Gear {
    char_x: usize,
    char_y: usize
}

impl Gear {
    fn new(char_x: usize, char_y: usize) -> Gear {
        Gear {
            char_x,
            char_y
        }
    }
}

impl fmt::Display for Gear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gear char_x: {}, char_y: {}, Surface: {}",self.char_x, self.char_y, self.get_surface())
    }
}

impl HasCoordinates for Gear {

    /// char_x - 1 because the surface of the Gear contains its surrounding
    fn get_x(&self) -> usize {
        self.char_x - 1
    }
    /// char_y - 1 because the surface of the Gear contains its surrounding
    fn get_y(&self) -> usize {
        self.char_y - 1
    }
    /// Gear has a fixed height of 3 (bot + * + top)
    fn get_height(&self) -> usize {
        2
    }
    /// Gear has a fixed height of 3 (left + * + right)
    fn get_widht(&self) -> usize {
        2
    }
    /// Returns a Surface that defines the Gear area, can then be used to check if intersection between 2 Surfaces
    fn get_surface(&self) -> Surface {
        Surface::new(self.get_x(), self.get_y(), self.get_widht(), self.get_height())
    }
}


fn get_input(filename: &str) -> Option<(Vec<PartNumber>, Vec<Gear>)> {
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

    // Read and process content to create an array of PartNumber
    // We will 'trick' the input by adding a fisrt and a last line full of '.' And full left and right columns full of '.'
    let mut vec_partnumber: Vec<PartNumber> = Vec::new(); 
    let mut vec_gear: Vec<Gear> = Vec::new(); 
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
                                let number_len = get_nb_digit(&number);

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
                                
                                vec_partnumber.push(PartNumber::new(number, number_len, index_huchar-number_len, index_line));
                                number = 0;
                                last_was_number = false;
                            }
                        }
                    }
                    if !last_was_number {
                        if huchar == '*' {
                            vec_gear.push(Gear::new(index_huchar, index_line))
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
    Some((vec_partnumber, vec_gear))
}


fn main() {
    // Process input as vec of structs
    let filename = "input.txt";
    let pn_vec: Vec<PartNumber>;
    let gear_vec: Vec<Gear>;
    match get_input(filename) {
        Some((pn_val, gear_val)) => {
            pn_vec = pn_val;
            gear_vec = gear_val;
        }
        None => {
            pn_vec = Vec::new();
            gear_vec = Vec::new();
        }
    }

    // Process vec of struct to result
    let mut result: u32 = 0;
    // Temporarily keeps Partnumber that intersects with the current gear
    let mut tmp_pn_vec: Vec<PartNumber>;

    for gear in &gear_vec {
        // Init/Reset the tmp vec
        tmp_pn_vec = Vec::new();
        // Push if intersects
        for pn in &pn_vec {
            if pn.value == 105 {
            }
            if gear.get_surface().surface_intersect(&pn.get_surface()) {
                tmp_pn_vec.push((*pn).clone());
            }
        }

        // Check if valid (exactly 2 partnumbers)
        if tmp_pn_vec.len() == 2 {
            result += tmp_pn_vec[0].value * tmp_pn_vec[1].value;
        }
        

    }
    println!("Result: {result}");
}

fn get_nb_digit(value: &u32) -> usize {
    let mut _value: u32 = *value;
    let mut number_len: usize = 1;
    while _value >= 10 {
        _value = _value / 10;
        number_len += 1;
    }
    number_len
}