use std::{env, fs, path::{Path, PathBuf}};

/// Represents a Game
#[derive(Debug)]
struct Game {
    id: u32,
    red: u8,
    green: u8,
    blue: u8
}

impl Game {
    fn new( id: u32, red: u8, green: u8, blue: u8) -> Game {
        Self {
            id,
            red: red,
            green: green,
            blue: blue,
        }
    }
}
/// Process a single line into a Game struct
fn get_line_data(line: &str) -> Option<Game> {
    let mut id: u32 = 0;
    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;

    for colon_split in line.split(':') {
        if colon_split.contains("Game") {
            match colon_split["Game".chars().count()..].trim().parse::<u32>() {
                Ok(matched_id) => {
                    id = matched_id;
                }
                Err(error) => {
                    eprintln!("Error while parsing a line to get the Game id: {error}");
                    return None;
                }
            }
        }
        else {
            for semicolon_split in colon_split.split(';') {
                    for coma_split in semicolon_split.split(',') {
                    if coma_split.contains("blue") {
                        match coma_split[1..coma_split.chars().count()-"blue".chars().count()-1].trim().parse::<u8>() {
                            Ok(val) => {
                                if val > blue {
                                    blue = val;
                                }
                            }
                            Err(error) => {
                                eprintln!("Error while getting the quantity of blue cubes: {error}");
                                return None;
                            }
                        }
                    }
                    else if coma_split.contains("red") {
                        match coma_split[1..coma_split.chars().count()-"red".chars().count()-1].trim().parse::<u8>() {
                            Ok(val) => {
                                if val > red {
                                    red = val;
                                }
                            }
                            Err(error) => {
                                eprintln!("Error while getting the quantity of red cubes: {error}");
                                return None;
                            }
                        }
                    }
                    else if coma_split.contains("green") {
                        match coma_split[1..coma_split.chars().count()-"green".chars().count()].trim().parse::<u8>() {
                            Ok(val) => {
                                if val > green {
                                    green = val;
                                }
                            }
                            Err(error) => {
                                eprintln!("Error while getting the quantity of green cubes: {error}");
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }
    Some(Game::new(id, red, green, blue))
}

/// Return the input file as vec of Games
fn get_input(filename: &str) -> Option<Vec<Game>> {
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
    let mut result: Vec<Game> = Vec::new(); 
    match fs::read_to_string(filepath) {
        Ok(content) => {
            for line in content.split('\n') {
                match get_line_data(line) {
                    Some(game) => {
                        result.push(game);
                    }
                    None => {
                        eprintln!("Error while getting the game, couldn't parse the line properly");
                        return None;
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error while opening the file: {error}");
            return None;
        }
    }
    Some(result)
}

fn main() {
    let filename = "input.txt";
    let vec_games: Vec<Game>;
    let nb_red: u8 = 12;
    let nb_green: u8 = 13;
    let nb_blue: u8 = 14;
    let mut result: u32 = 0;


    match get_input(filename) {
        Some(games) => {
            vec_games = games;
        }
        None => {
            eprintln!("Error while getting the games");
            std::process::exit(1);
        }
    }
    for game in vec_games {
        if game.red <= nb_red && game.green <= nb_green && game.blue <= nb_blue {
            result += game.id;
        }
    }
    println!("The result is: {}", result);
}
