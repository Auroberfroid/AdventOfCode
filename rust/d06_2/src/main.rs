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
    time: usize,
    record_distance: usize
}

impl BoatRace {
    fn new(id: usize, time: usize, record_distance: usize) -> Self {
        Self {
            id,
            time,
            record_distance
        }
    }
}


fn get_input(filename: &str) -> Result<BoatRace, String> {
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


    let mut distance: usize = 0;
    let mut time: usize = 0;
    for line in content.split('\n') {
        if line.contains("Time:") {
            let time_str = line.replace("Time:", "").trim().replace(" ", "");
            match time_str.parse::<usize>() {
                Ok(val) => {
                    time = val;
                }
                Err(error) => {
                    let err_msg = format!("Error while parsing the time value: '{time}'. Error: '{error}'");
                    eprintln!("{err_msg}");
                    return Err(err_msg);
                }
            }
        }
        else if line.contains("Distance:") {
            let distance_str = line.replace("Distance:", "").trim().replace(" ", "");
            match distance_str.parse::<usize>() {
                Ok(val) => {
                    distance = val;
                }
                Err(error) => {
                    let err_msg = format!("Error while parsing the distance value: '{distance}'. Error: '{error}'");
                    eprintln!("{err_msg}");
                    return Err(err_msg);
                }
            }
        }
    }

    let mut boat_race = BoatRace::new(0, time, distance);

    Ok(boat_race)
}


fn process_boat_race(boat_race: &BoatRace) -> usize {
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

    let boat_race: BoatRace;
    match get_input(filename ) {
        Ok(val) => {
            boat_race = val;
        }
        Err(_) => {
            return Err(-1);
        }
    }

    let result: usize = process_boat_race(&boat_race);

    println!("Result is {result}");

    Ok(())
}
