use std::{env::current_dir, fs::read_to_string, path::{Path, PathBuf}};
use regex::Regex;

const DEBUG: bool = false;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AlmanacDescr
{
    Unknown,
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
    LastTable
}

#[derive(Debug, Clone)]
struct AlmanacRange 
{
    start: u64,
    end: u64,
}

impl AlmanacRange 
{
    fn new(start: u64, end: u64) -> AlmanacRange {
        AlmanacRange {
            start,
            end,
        }
    }
} 

#[derive(Debug, Clone)]
struct AlmanacEntry
{
    desc_src: AlmanacDescr,
    desc_dst: AlmanacDescr,
    ranges_src: Vec<AlmanacRange>,
    ranges_dst: Vec<AlmanacRange>
}

impl AlmanacEntry {
    fn new(desc_src: AlmanacDescr, desc_dst: AlmanacDescr, ranges_src: Vec<AlmanacRange>, ranges_dst: Vec<AlmanacRange>) -> Self {
        Self {
            desc_src,
            desc_dst,
            ranges_src,
            ranges_dst
        }
    }

    fn get_dst(&self, src: u64) -> u64 {
        let mut i: usize = 0;
        for alma_range_src in &self.ranges_src {
            if alma_range_src.start <= src && src <= alma_range_src.end {
                return self.ranges_dst[i].start + src - alma_range_src.start;
            }
            i += 1;
        }
        src
    }
}

fn get_input(filename: &str) -> Result<(Vec<AlmanacEntry>, Vec<u64>), &'static str>
{
    let filepath: PathBuf;
    let input_dir: &str = "files";
    match current_dir() {
        Ok(wd) => {
            filepath = wd.join(Path::new(input_dir)).join(Path::new(filename));
        }
        Err(error) => {
            eprintln!("Error while getting the current working directory: {error}");
            return Err("Error while getting the current working directory");
        }
    }

    let almanac_descriptions: [(AlmanacDescr, &str); 8] = [ (AlmanacDescr::Seed, "seed"),
                                                            (AlmanacDescr::Soil, "soil"),
                                                            (AlmanacDescr::Fertilizer, "fertilizer"),
                                                            (AlmanacDescr::Water, "water"),
                                                            (AlmanacDescr::Light, "light"),
                                                            (AlmanacDescr::Temperature, "temperature"),
                                                            (AlmanacDescr::Humidity, "humidity"),
                                                            (AlmanacDescr::Location, "location")];

    let new_almanac_entry_regex_pattern: Regex;
    match Regex::new(r".+?-to-.+? map:") {
        Ok(val) => {
            new_almanac_entry_regex_pattern = val;
        }
        Err(error) => {
            eprintln!("Error while defining the Regex expression pattern to match for map: {error}");
            return Err("Error while defining the Regex expression pattern to match for map");
        }
    }

    let seeds_entry_regex_pattern: Regex;
    match Regex::new(r"seeds:") {
        Ok(val) => {
            seeds_entry_regex_pattern = val;
        }
        Err(error) => {
            eprintln!("Error while defining the Regex expression pattern to match for seeds: {error}");
            return Err("Error while defining the Regex expression pattern to match for seeds");
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

    let mut desc_src: AlmanacDescr = AlmanacDescr::Unknown;
    let mut desc_dst: AlmanacDescr = AlmanacDescr::Unknown;
    let mut ranges_src: Vec<AlmanacRange> = Vec::new();
    let mut ranges_dst: Vec<AlmanacRange> = Vec::new();
    let mut seeds_entry: Vec<u64> = Vec::new();
    let mut almanac_entries: Vec<AlmanacEntry> = Vec::new();
    let mut add_alma_entry: bool = false;

    for line in content.split('\n') {
        if line == "" && desc_src != AlmanacDescr::Unknown {
            add_alma_entry = true;
        }

        // Get seeds
        if seeds_entry_regex_pattern.is_match(line) {
            for seed in line.replace("seeds: ", "").split_whitespace() {
                if seed != " " {
                    match seed.trim().parse::<u64>() {
                        Ok(val) => {
                            seeds_entry.push(val);
                        }
                        Err(error) => {
                            eprintln!("Error while parsing the seed entry into a u64: seed: '{seed}' Error: {error}");
                            return Err("Error while parsing the seed entry into a u64");
                        }
                    }
                }
            }
            continue;
        }

        // New Almanac Entry
        if new_almanac_entry_regex_pattern.is_match(line) {
            ranges_src = Vec::new();
            ranges_dst = Vec::new();
            let to_parse_line = line.replace(" map:", "");
            for (i, almanac_map_desc_entry) in to_parse_line.split("-to-").enumerate() {
                for almanac_desc in &almanac_descriptions {
                    if almanac_map_desc_entry == almanac_desc.1 {
                        // Src descr
                        if i % 2 == 0 {
                            desc_src = almanac_desc.0;
                        }
                        // Dst descr
                        else {
                            desc_dst = almanac_desc.0;

                        }
                    }
                }
            }
            continue;
        }
        // Almanac entry mapping
        else {           
            let mut dst_start: u64 = 0;
            let mut src_start: u64 = 0;
            let mut lenght: u64 = 0;
            for (i, alma_entry_detail) in line.split_whitespace().enumerate() {
                // Dst start
                if i % 3 == 0 {
                    match alma_entry_detail.trim().parse::<u64>() {
                        Ok(val) => {
                            dst_start = val;
                        }
                        Err(error) => {
                            eprintln!("Error while parsing the dst start. dst_start: '{alma_entry_detail}' Error: {error}");
                            return Err("Error while parsing the dst start.");
                        }
                    }
                }

                // Src start
                if i % 3 == 1 {
                    match alma_entry_detail.trim().parse::<u64>() {
                        Ok(val) => {
                            src_start = val;
                        }
                        Err(error) => {
                            eprintln!("Error while parsing the src start. src_start: '{alma_entry_detail}' Error: {error}");
                            return Err("Error while parsing the src start.");
                        }
                    }
                }

                // Lenght
                if i % 3 == 2 {
                    match alma_entry_detail.trim().parse::<u64>() {
                        Ok(val) => {
                            lenght = val;
                        }
                        Err(error) => {
                            eprintln!("Error while parsing the lenght. lenght: '{alma_entry_detail}' Error: {error}");
                            return Err("Error while parsing the lenght.");
                        }
                    }
                }
            }
            match desc_src {
                AlmanacDescr::Unknown => {
                    continue;
                }
                _ => {
                    let range_src: AlmanacRange = AlmanacRange::new(src_start, src_start + lenght);
                    let range_dst: AlmanacRange = AlmanacRange::new(dst_start, dst_start + lenght);
                    ranges_src.push(range_src);
                    ranges_dst.push(range_dst);
                }
            }
        }
        if add_alma_entry {
            // Create the AlmanacEntry if needed
            let almanac_entry: AlmanacEntry = AlmanacEntry::new(desc_src, desc_dst, ranges_src.clone(), ranges_dst.clone());
            almanac_entries.push(almanac_entry);
            add_alma_entry = false;
        }
    }

    // Add the last AlmanacEntry
    let almanac_entry: AlmanacEntry = AlmanacEntry::new(desc_src, desc_dst, ranges_src.clone(), ranges_dst.clone());
    almanac_entries.push(almanac_entry);
    add_alma_entry = false;

    Ok((almanac_entries, seeds_entry))
}

fn get_next_alma_desc(current_alma_desc: AlmanacDescr) -> AlmanacDescr {
    match current_alma_desc {
        AlmanacDescr::Seed => {
            AlmanacDescr::Soil
        }
        AlmanacDescr::Soil => {
            AlmanacDescr::Fertilizer
        }
        AlmanacDescr::Fertilizer => {
            AlmanacDescr::Water
        }
        AlmanacDescr::Water => {
            AlmanacDescr::Light
        }
        AlmanacDescr::Light => {
            AlmanacDescr::Temperature
        }
        AlmanacDescr::Temperature => {
            AlmanacDescr::Humidity
        }
        AlmanacDescr::Humidity => {
            AlmanacDescr::Location
        }
        AlmanacDescr::Location => {
            AlmanacDescr::LastTable
        }
        AlmanacDescr::LastTable => {
            AlmanacDescr::LastTable
        }
        AlmanacDescr::Unknown => {
            AlmanacDescr::Unknown
        }
    }
}

fn get_alma_entry_index_from_desc(alma_entries: &Vec<AlmanacEntry>, alma_desc: AlmanacDescr) -> Option<usize> {
    let mut i: usize = 0;
    for alma_entry in alma_entries {
        if alma_entry.desc_src == alma_desc {
            return Some(i);
        }
        i += 1;
    }
    return None;
}


fn main() 
{
    let filename: &str;
    let almanac_entries: Vec<AlmanacEntry>;
    let seeds: Vec<u64>;
    if DEBUG {
        filename = "input_debug.txt";
    }
    else {
        filename = "input.txt";
    }
    match get_input(filename) {
        Ok(val) => {
            almanac_entries = val.0;
            seeds = val.1;
        }
        Err(error) => {
            eprintln!("Error while getting the input. Error {error}");
            almanac_entries = Vec::new();
            seeds = Vec::new();
        }
    }
    
    // for alma_entry in &almanac_entries {
    //     println!("{:?}", alma_entry);
    //     println!("##################################################################");
    // }
    
    let mut alma_entry_index: usize = 0;
    let mut location_result: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut seed_result: u64 = 0;
    for seed in seeds {
        let mut current_alma_id = seed;
        let mut current_alma_desc = AlmanacDescr::Seed;
        while current_alma_desc != AlmanacDescr::LastTable {
            match get_alma_entry_index_from_desc(&almanac_entries, current_alma_desc) {
                Some(val) => {
                    alma_entry_index = val;
                }
                None => {
                    // eprintln!("No Almanac Entry found for src_desc: {:?}", current_alma_desc);
                    break;
                }
            }

            current_alma_id = almanac_entries[alma_entry_index].get_dst(current_alma_id);
            current_alma_desc = get_next_alma_desc(current_alma_desc);
        }
        if current_alma_id < location_result {
            seed_result = seed;
            location_result = current_alma_id;
        }
    }
    println!("Lowest location from seed is: {seed_result}");
    println!("Lowest location is: {location_result}");
}
