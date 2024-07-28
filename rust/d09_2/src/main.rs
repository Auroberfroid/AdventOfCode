use std::{env::current_dir, fs::read_to_string, path::PathBuf};

const DEBUG: bool = false;

fn get_input(filename: &str) -> Result<Vec<Vec<i64>>, String> {

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

    // Parse the input and push vec of usize to the vec
    let mut sequences = Vec::<Vec::<i64>>::new();
    for line in content.lines() {
        let mut sequence = Vec::<i64>::new();
        for nb in line.split_whitespace() {
            match nb.parse::<i64>() {
                Ok(val) => {
                    sequence.push(val);
                }
                Err(error) => {
                    let err_msg = format!("[Error while parsing the string ('{nb}') into i64: {error}]");
                    return Err(err_msg);
                }
            }
        }
        sequences.push(sequence);
    }

    Ok(sequences)
}

fn process_sequence(sequence: &Vec<i64>) -> i64 {
    let mut finished = false;
    let mut current_sequence = sequence.clone();
    let mut current_nb: i64 = 0;
    let mut last_nb: i64;
    let mut processed_sequences = Vec::<Vec::<i64>>::new();
    
    // Process until only 0 in next sequence
    while !finished {
        // Process the next sequence    
        let mut next_sequence = Vec::<i64>::new();
        for (index_nb, nb) in current_sequence.clone().into_iter().enumerate() {
            if index_nb == 0 {
                current_nb = nb;
                continue;
            }
            else {
                last_nb = current_nb;
                current_nb = nb;
            }
            next_sequence.push(current_nb - last_nb);
        }
        
        // Check if contains only 0
        let mut check = true;
        for nb in &next_sequence {
            match nb {
                0 => {
                    check = check && check;
                }
                _ => {
                    check = false;
                }
            }
        }
        finished = check;
        
        // Keep all processed sequences in vec
        processed_sequences.push(current_sequence);
        current_sequence = next_sequence;
    }

    // Now extrapolate from processed sequences
    processed_sequences.reverse();
    let mut last_seq_first_nb: i64 = 0;
    let mut current_seq_prev_first_nb: i64;
    let mut current_seq_first_nb: i64 = 0;
    for processed_seq in &processed_sequences {
        current_seq_prev_first_nb = *processed_seq.get(0).unwrap();
        current_seq_first_nb = current_seq_prev_first_nb - last_seq_first_nb;
        last_seq_first_nb = current_seq_first_nb;
    }
    current_seq_first_nb
}


fn main() -> Result<(), i8> {
    let filename: &str;
    if DEBUG {
        filename = "input_debug.txt";
    }
    else {
        filename = "input.txt";
    }

    let sequences: Vec::<Vec::<i64>>;
    match get_input(filename) {
        Ok(val) => {
            sequences = val;
        }
        Err(error) => {
            let err_msg = format!("[Error while get_input: {error}]");
            eprintln!("{}", err_msg);
            return Err(-1);
        }
    }


    println!("sequences: {:?}", sequences);
    
    let mut res: i64 = 0;
    for seq in sequences {
        res += process_sequence(&seq);
    }

    println!("result: {}", res);

    Ok(())
}
