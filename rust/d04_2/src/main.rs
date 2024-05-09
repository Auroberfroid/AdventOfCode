use std::{env, fmt::Debug, fs::read_to_string, hash::Hash, path::{Path, PathBuf}, cmp::min};
use hashbrown::HashSet;

const DEBUG: bool = false;

const SIZE_NB_CARDS: usize = 25;
const SIZE_NB_WINNINGS: usize = 10;
const SIZE_DEBUG_NB_CARDS: usize = 8;
const SIZE_DEBUG_NB_WINNINGS: usize = 5;

fn get_input<T>(filename: &str) -> Result<Vec<T>, &'static str> 
where T: CardGenerics + Sized
{
    let filepath: PathBuf;
    let input_dir: &str = "files";
    match env::current_dir() {
        Ok(wd) => {
            filepath = wd.join(Path::new(input_dir)).join(Path::new(filename));
        }
        Err(error) => {
            eprintln!("Error while getting the current working directory: {error}");
            return Err("Error while getting the current working directory");
        }
    }

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
            }
            else {
                let mut i: u8 = 0;
                for pipe_sep in colon_sep.split('|') {
                    if i % 2 == 0 {
                        for space_sep in pipe_sep.split_whitespace() {
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
            Ok(mut vec) => {
                let vec_len = vec.len();
                for index_card in 0..vec_len {
                    let score = vec[index_card].get_score();
                    for i in vec[index_card].id+1..vec[index_card].id+score+1 {
                        if i >= vec_len {
                            break;
                        }
                        vec[i-1].amount += vec[index_card].amount;
                    }
                }
                for card in vec {
                    result += card.amount;
                }
                println!("Result is {result}");
            }
            Err(error) => {
                eprintln!("Error while getting the input from files {error}");
            }
        }
    }
    else {
        filename = "input.txt";
        match get_input::<Card>(filename) {
            Ok(mut vec) => {
                let vec_len = vec.len();
                for index_card in 0..vec_len {
                    let amount_matches = vec[index_card].get_amount_matches();
                    for i in vec[index_card].id..min::<usize>(vec[index_card].id + amount_matches, vec_len) {
                        vec[i].amount += vec[index_card].amount;
                    }
                }
                for card in vec {
                    result += card.amount;
                }
                println!("Result is {result}");
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

    fn get_amount_matches(&self) -> usize {
        get_intersection([self.get_card_nbs(), self.get_winning_nbs()]).len()
    }

    fn get_score(&self) -> usize {
        let mut score: usize = 0;
        let amount_matches = self.get_amount_matches();
        if amount_matches > 0 {
            score = 1;
            for _ in 1..amount_matches {
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

#[derive(Debug, Clone)]
struct DebugCard {
    id: usize,
    winning_nbs: [u8; SIZE_DEBUG_NB_WINNINGS],
    card_nbs: [u8; SIZE_DEBUG_NB_CARDS],
    amount: usize
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
        let amount: usize = 1;
        let res: Self = Self {id, winning_nbs, card_nbs, amount};
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

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_nbs: [u8; SIZE_NB_WINNINGS],
    card_nbs: [u8; SIZE_NB_CARDS],
    amount: usize
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
        let amount: usize = 1;
        let res: Self = Self {id, winning_nbs, card_nbs, amount};
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