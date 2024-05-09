use std::{env, fmt::Debug, fs::read_to_string, hash::Hash, path::{Path, PathBuf}};
use hashbrown::HashSet;

const DEBUG: bool = false;

const SIZE_NB_CARDS: usize = 25;
const SIZE_NB_WINNINGS: usize = 10;
const SIZE_DEBUG_NB_CARDS: usize = 8;
const SIZE_DEBUG_NB_WINNINGS: usize = 5;

fn get_input<T>(filename: &str) -> Result<Vec<T>, &'static str> 
where T: CardGenerics + Sized
{
    let mut filepath: PathBuf = PathBuf::new();
    match env::current_dir() {
        Ok(wd) => {
            filepath = wd;
        }
        Err(error) => {
            eprintln!("Error while getting the current working directory: {error}");
            return Err("Error while getting the current working directory");
        }
    }
    let input_dir: &str = "files";
    filepath = filepath.join(Path::new(input_dir)).join(Path::new(filename));

    let content: String;
    match read_to_string(filepath) {
        Ok(val) => {
            content = val;
        }
        Err(error) => {
            eprint!("Error while reading the file: {error}");
            return Err("Error while reading the file");
        }
    }

    let mut cards: Vec<T> = Vec::new();

    for line in content.split('\n') {
        let mut id: usize = 0;
        let mut vec_winning_nbs: Vec<u8> = Vec::new();
        let mut vec_card_nbs: Vec<u8> = Vec::new();
        
        for colon_sep in line.split(':') {
            // println!("ARO DEBUG from get_input: colon_sep: {colon_sep}");
            if colon_sep.contains("Card") {
                match colon_sep.replace("Card ", "").trim().parse::<usize>() {
                    Ok(val) => {
                        id = val;
                    }
                    Err(error) => {
                        eprintln!("Error while parsing the card id. Error: {error}");
                        return Err("Error while parsing the card id");
                    }
                }
                // println!("ARO DEBUG from get_input: card id: {}", id);
            }
            else {
                let mut i: u8 = 0;
                for pipe_sep in colon_sep.split('|') {
                    // println!("ARO DEBUG from get_input: looping through the pipe_sep");
                    if i % 2 == 0 {
                        for space_sep in pipe_sep.split_whitespace() {
                            // println!("ARO DEBUG from get_input: space_sep: '{space_sep}'");
                            match space_sep.trim().parse::<u8>() {
                                Ok(val) => {
                                    vec_winning_nbs.push(val);
                                }
                                Err(error) => {
                                    eprintln!("Error while parsing the winning numbers for id: {id}. Error: {error}");
                                    return Err("Error while parsing the winning numbers");
                                }
                            }
                        }
                    } else {
                        for space_sep in pipe_sep.split_whitespace() {
                            // println!("ARO DEBUG from get_input: space_sep: '{space_sep}'");
                            match space_sep.trim().parse::<u8>() {
                                Ok(val) => {
                                    vec_card_nbs.push(val);
                                }
                                Err(error) => {
                                    eprintln!("Error while parsing the card numbers for id: {id}. Error: {error}");
                                    return Err("Error while parsing the card numbers");
                                }
                            }
                        }
                    }
                    i += 1;
                    if i == 0xFF {
                        i = 1
                    }
                }
            }
        }
        let card: T;
        // println!("ARO DEBUG from get_input: vec_winning_nbs: {:?}", vec_winning_nbs);
        // println!("ARO DEBUG from get_input: vec_card_nbs: {:?}", vec_card_nbs);
        match get_card::<T>(id, vec_winning_nbs.clone(), vec_card_nbs.clone()) {
            Ok(val) => {
                card = val;
            }
            Err(error) => {
                eprintln!("Error while getting new card instance: id {id}. Error: {error}");
                return Err("Error while getting new card instance.");
            }
        }
        cards.push(card);
    }
    Ok(cards)
}


fn main() {
    let filename: &str;
    let mut result: usize = 0;
    if DEBUG {
        filename = "input_debug.txt";
        match get_input::<DebugCard>(filename) {
            Ok(vec) => {
                for card in vec {
                    let score = card.get_score();
                    for i in card.id..card.id+score {

                    }
                    result += score;
                }
            }
            Err(error) => {
                eprintln!("Error while getting the input from files {error}");
            }
        }
    }
    else {
        filename = "input.txt";
        match get_input::<Card>(filename) {
            Ok(val) => {
                for vec in val {
                    result += vec.get_score();
                }
            }
            Err(error) => {
                eprintln!("Error while getting the input from files {error}");
            }
        }
    }
    println!("Result: {result}");
}


trait CardGenerics {
    fn new(id: usize, vec_winning_nbs: Vec<u8>, vec_card_nbs: Vec<u8>) -> Result<Self, &'static str>
    where Self: Sized;

    fn get_size_card_nbs() -> usize;
    fn get_size_winning_nbs() -> usize;

    fn get_card_nbs(&self) -> Vec<u8>;
    fn get_winning_nbs(&self) -> Vec<u8>;

    fn get_score(&self) -> usize {
        let intersection_vec:Vec<u8> = get_intersection([self.get_card_nbs(), self.get_winning_nbs()]);
    
        let mut score: usize = 0;
        if intersection_vec.len() > 0 {
            score = 1;
            for _ in 1..intersection_vec.len() {
                score = score << 1;
            }
        }
        score
    }
}

pub fn get_intersection<T>(nums: [Vec<T>; 2]) -> Vec<T>
where T: Eq + PartialEq + Hash + Clone + Copy
{
    let mut intersect_result: Vec<T> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<T> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    intersect_result
}

#[derive(Debug)]
struct DebugCard {
    id: usize,
    winning_nbs: [u8; SIZE_DEBUG_NB_WINNINGS],
    card_nbs: [u8; SIZE_DEBUG_NB_CARDS],
}

impl CardGenerics for DebugCard {
    fn new(id: usize, vec_winning_nbs: Vec<u8>, vec_card_nbs: Vec<u8>) -> Result<Self, &'static str> {
        let winning_nbs: [u8; SIZE_DEBUG_NB_WINNINGS];
        // println!("ARO DEBUG: from new: vec_winning_nbs: {:?}", vec_winning_nbs);
        // println!("ARO DEBUG: from new: vec_card_nbs: {:?}", vec_card_nbs);
        match vec_winning_nbs.try_into() {
            Ok(array) => {
                winning_nbs = array;
            }
            Err(_) => {
                return Err("Error while converting winning numbers vec into array [DEBUG mode]");
            }
        }
        let card_nbs: [u8; SIZE_DEBUG_NB_CARDS];
        match vec_card_nbs.try_into() {
            Ok(array) => {
                card_nbs = array;
            }
            Err(_) => {
                return Err("Error while converting card numbers vec into array [DEBUG mode]");
            }
        }
        let res: Self = Self {id, winning_nbs, card_nbs};
        return Ok(res)
    }

    fn get_size_card_nbs() -> usize {
        SIZE_DEBUG_NB_CARDS
    }
    fn get_size_winning_nbs() -> usize {
        SIZE_DEBUG_NB_WINNINGS
    }
    fn get_card_nbs(&self) -> Vec<u8> {
        self.card_nbs.to_vec()
    }
    fn get_winning_nbs(&self) -> Vec<u8> {
        self.winning_nbs.to_vec()
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nbs: [u8; SIZE_NB_WINNINGS],
    card_nbs: [u8; SIZE_NB_CARDS],
}

impl CardGenerics for Card {
    fn new(id: usize, vec_winning_nbs: Vec<u8>, vec_card_nbs: Vec<u8>) -> Result<Self, &'static str> {
        let winning_nbs: [u8; SIZE_NB_WINNINGS];
        match vec_winning_nbs.try_into() {
            Ok(array) => {
                winning_nbs = array;
            }
            Err(_) => {
                return Err("Error while converting winning numbers vec into array");
            }
        }
        let card_nbs: [u8; SIZE_NB_CARDS];
        match vec_card_nbs.try_into() {
            Ok(array) => {
                card_nbs = array;
            }
            Err(_) => {
                return Err("Error while converting card numbers vec into array");
            }
        }
        let res: Self = Self {id, winning_nbs, card_nbs};
        return Ok(res)
    }

    fn get_size_card_nbs() -> usize {
        SIZE_NB_CARDS
    }
    fn get_size_winning_nbs() -> usize {
        SIZE_NB_WINNINGS
    }
    fn get_card_nbs(&self) -> Vec<u8> {
        self.card_nbs.to_vec()
    }
    fn get_winning_nbs(&self) -> Vec<u8> {
        self.winning_nbs.to_vec()
    }
}


fn get_card<T>(id: usize, vec_winning_nbs: Vec<u8>, vec_card_nbs: Vec<u8>) -> Result<T, &'static str>
where T: CardGenerics + Sized
{
    let card: T;
    // println!("ARO DEBUG from get_card: vec_winning_nbs: {:?}", vec_winning_nbs);
    // println!("ARO DEBUG from get_card: vec_card_nbs: {:?}", vec_card_nbs);
    match T::new(id, vec_winning_nbs, vec_card_nbs) {
        Ok(val) => {
            card = val;
        }
        Err(error) => {
            eprintln!("Error while getting card {error}");
            return Err("Error while getting card");
        }
    }
    Ok(card)
}