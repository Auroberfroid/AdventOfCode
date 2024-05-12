use std::{env::current_dir, fs::read_to_string, path::PathBuf, cmp::Ordering};

const DEBUG: bool = false;


#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
enum Card {
    A,
    K,
    Q,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
    Unknown
}

fn get_card_variants() -> [(Card, char); 13] {
    [(Card::A, 'A'), (Card::K, 'K'), (Card::Q, 'Q'), (Card::J, 'J'), (Card::Ten, 'T'), (Card::Nine, '9'), (Card::Eight, '8'), (Card::Seven, '7'), (Card::Six, '6'), (Card::Five, '5'), (Card::Four, '4'), (Card::Three, '3'), (Card::Two, '2')]
}

fn get_card_from_char(card_char: &char) -> Result<Card, String> {
    for card in &get_card_variants() {
        if &card.1 == card_char {
            return Ok(card.0);
        }
    }
    let err_msg = format!("[Couldn't get the card from the char: '{card_char}']");
    Err(err_msg)
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum ComboType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Unknown
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    rank: usize,
    combo_type: ComboType
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let mut self_instance = Self {
            cards,
            bid,
            rank: 0,
            combo_type: ComboType::Unknown
        };

        self_instance.set_combo_type();
        self_instance
    }

    fn get_combo_type(&self) -> Result<ComboType, String> {
        let mut card_buckets = Vec::<(Card, usize)>::new();
        let mut tmp_nb: usize;
        let mut total_nb: usize = 0;
        for card_ref in &get_card_variants() {
            tmp_nb = self.cards.iter().filter(|&card| card == &card_ref.0).count();
            total_nb += tmp_nb;
            if tmp_nb > 0 {
                card_buckets.push((card_ref.0, tmp_nb));
            }
            if total_nb == 5 {
                break;
            }
        }
        
        let mut largest_bucket_card: Card = Card::Unknown;
        let mut largest_bucket_size: usize = 0;
        let mut contains_joker = false;
        let mut joker_bucket_size: usize = 0;
        let mut joker_bucket_index: usize = 0;
        for (c_bucket_index, c_bucket) in card_buckets.iter().enumerate() {
            if c_bucket.0 != Card::J {
                if c_bucket.1 > largest_bucket_size {
                    largest_bucket_size = c_bucket.1;
                    largest_bucket_card = c_bucket.0;
                }
                else if c_bucket.1 == largest_bucket_size {
                    if c_bucket.0 < largest_bucket_card {
                        largest_bucket_size = c_bucket.1;
                        largest_bucket_card = c_bucket.0;
                    }
                }
            }
            else {
                contains_joker = true;
                joker_bucket_size = c_bucket.1;
                joker_bucket_index = c_bucket_index;
            }
        }

        if contains_joker {
            card_buckets.remove(joker_bucket_index);
            if largest_bucket_card != Card::Unknown {
                for c_bucket in card_buckets.iter_mut() {
                    if c_bucket.0 == largest_bucket_card {
                        c_bucket.1 += joker_bucket_size;
                    }
                }
            }
            else {
                return Ok(ComboType::FiveOfAKind); // case of 5J
            }
        }

        match card_buckets.len() {
            5 => {
                return Ok(ComboType::HighCard);
            }
            4 => {
                return Ok(ComboType::OnePair);
            }
            3 => {
                for card_bucket_size in &card_buckets {
                    if card_bucket_size.1 == 3 {
                        return Ok(ComboType::ThreeOfAKind);
                    }
                }
                return Ok(ComboType::TwoPair);
            }
            2 => {
                if card_buckets[0].1 == 4 ||card_buckets[0].1 == 1 {
                    return Ok(ComboType::FourOfAKind);
                }
                else if card_buckets[0].1 == 3 ||card_buckets[0].1 == 2 {
                    return Ok(ComboType::FullHouse);
                }
                else {
                    let err_msg = format!("[Could not define hand type: {:?}]", card_buckets);
                    return Err(err_msg);
                }
            }
            1 => {
                return Ok(ComboType::FiveOfAKind);
            }
            _ => {
                    let err_msg = format!("[Could not define hand type: {:?}]", card_buckets);
                    return Err(err_msg);
            }
        }
    }

    fn set_combo_type(&mut self) {
        match self.get_combo_type() {
            Ok(val) => {
                self.combo_type = val;
            }
            Err(error) => {
                eprintln!("[Couldn't set the combo type for the hand: {:?}. Error: {error}]", self);
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> { 
        if self.combo_type < other.combo_type {
            return Some(Ordering::Greater);
        }
        else if self.combo_type > other.combo_type {
            return Some(Ordering::Less);
        }
        else {
            let mut card_index: usize = 0;
            while card_index < self.cards.len() {
                if self.cards[card_index] < other.cards[card_index] {
                    return Some(Ordering::Greater);
                }
                else if self.cards[card_index] > other.cards[card_index] {
                    return Some(Ordering::Less);
                }
                else {
                    card_index += 1;
                }
            }
        }

        None
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering { 
        match self.partial_cmp(other) {
            Some(val) => {
                return val;
            }
            None => {
                eprintln!("Comparison didn't held in partial_cmp between: self: {:?}, other: {:?}", self, other);
                return Ordering::Greater;
            }
        }
    }
}


fn get_input(filename: &str) -> Result<Vec<Hand>, String> {
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

    let mut hands = Vec::<Hand>::new();
    for line in content.split('\n') {
        let mut i: u8 = 0;
        let mut cards: [Card; 5] = [Card::Unknown; 5];
        let mut bid: usize = 0;
        for white_char_split in line.trim().split_whitespace() {
            if i % 2 == 0 {
                for (index_card, card_char) in white_char_split.chars().enumerate() {
                    match get_card_from_char(&card_char) {
                        Ok(val) => {
                            cards[index_card] = val;
                            // println!("ARO DEBUG: added val: {:?} to index: {index_card}", val);
                        }
                        Err(error) => {
                            let err_msg = format!("[No match found for: '{card_char}'. Error: {error}]");
                            return Err(err_msg);
                        }
                    }
                }
            }
            else {
                match white_char_split.parse::<usize>() {
                    Ok(val) => {
                        bid = val;
                    }
                    Err(error) => {
                        let err_msg = format!("Error while parsing the bid: '{white_char_split}'. Error: {error}");
                        return Err(err_msg); 
                    }
                }
                hands.push(Hand::new(cards, bid));
            }
            if i == 0xFF {
                i = 0;
            }
            else {
                i += 1;
            }
        }
    }
    Ok(hands)
}

fn process_hands_rank(hands: &mut Vec<Hand>) {
    hands.sort();
    for (rank_index, hand) in hands.iter_mut().enumerate() {
        hand.rank = rank_index + 1;
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

    let mut hands: Vec<Hand>;
    match get_input(filename) {
        Ok(val) => {
            hands = val;
        }
        Err(error) => {
            eprintln!("Error while getting the input data: {error}");
            return Err(-1);
        }
    }

    process_hands_rank(&mut hands);

    let mut result: usize = 0;
    for hand in &hands {
        result += hand.bid * hand.rank;
    }

    println!("The result is: {result}");

    // println!("####################################################");
    // for hand in &hands {
    //     println!("{:?}", hand);
    //     println!("####################################################");
    // }



    Ok(())
}
