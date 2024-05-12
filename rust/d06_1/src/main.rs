use std::{env::current_dir, fmt::Display, fs::read_to_string, path::PathBuf};

const DEBUG: bool = false;

fn debug_print<T: Display>(arg: T) {
    if DEBUG {
        println!("ARO DEBUG: {arg}");
    }
}

#[derive(Debug)]
struct BoatRace {
    id: usize,
    time: u16,
    record_distance: u16
}

impl BoatRace {
    fn new(id: usize, time: u16, record_distance: u16) -> Self {
        Self {
            id,
            time,
            record_distance
        }
    }
}


fn get_input(filename: &str) -> Result<Vec<BoatRace>, String> {
    let filepath: PathBuf;
    let input_dir: &str = "files";
    match current_dir() {
        Ok(val) => {
            filepath = val.join(input_dir).join(filename);
        }
        Err(error) => {
            let err_msg = format!("Error while getting the input file. Error: '{error}'");
            eprintln!("{err_msg}");
            return Err(err_msg);
        }
    }

    let content: String;
    match read_to_string(filepath) {
        Ok(val) => {
            content = val;
        }
        Err(error) => {
            let err_msg = format!("Error while reading the input file. Error: '{error}'");
            eprintln!("{err_msg}");
            return Err(err_msg);
        }
    }


    let mut time_values: Vec<u16> = Vec::new();
    let mut distance_values: Vec<u16> = Vec::new();
    for line in content.split('\n') {
        if line.contains("Time:") {
            for time in line.replace("Time:", "").trim().split_whitespace() {
                match time.parse::<u16>() {
                    Ok(val) => {
                        time_values.push(val);
                    }
                    Err(error) => {
                        let err_msg = format!("Error while parsing the time value: '{time}'. Error: '{error}'");
                        eprintln!("{err_msg}");
                        return Err(err_msg);
                    }
                }
            }
        }
        else if line.contains("Distance:") {
            for distance in line.replace("Distance:", "").trim().split_whitespace() {
                match distance.parse::<u16>() {
                    Ok(val) => {
                        distance_values.push(val);
                    }
                    Err(error) => {
                        let err_msg = format!("Error while parsing the distance value: '{distance}'. Error: '{error}'");
                        eprintln!("{err_msg}");
                        return Err(err_msg);
                    }
                }
            }
        }
    }

    let mut boat_vec: Vec<BoatRace> = Vec::new();
    let mut id: usize = 0;
    for (time, distance) in time_values.iter().zip(distance_values.iter()) {
        boat_vec.push(BoatRace::new(id, *time, *distance));
        id += 1;
    }

    Ok(boat_vec)
}


fn process_boat_races(boat_race: &BoatRace) -> usize {
    let mut nb_wins: usize = 0;
    for charging_time in 0..boat_race.time {
        let traveled_distance = (boat_race.time - charging_time) * charging_time;
        if traveled_distance > boat_race.record_distance {
            nb_wins += 1;
        }
    }
    nb_wins
}


fn main() -> Result<(), i8>{
    let filename: &str;
    if DEBUG {
        filename = "input_debug.txt";
    }
    else {
        filename = "input.txt";
    }

    let boatrace_vec: Vec<BoatRace>;
    match get_input(filename ) {
        Ok(val) => {
            boatrace_vec = val;
        }
        Err(_) => {
            return Err(-1);
        }
    }

    let mut result: usize = 1;
    for boat_race in boatrace_vec.iter() {
        println!("{:?}", boat_race);
        let nb_wins = process_boat_races(boat_race);
        if nb_wins > 0 {
            result *= nb_wins;
        }
    }

    println!("Result is {result}");

    Ok(())
}
