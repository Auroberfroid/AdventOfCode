use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::ops::{Add, Sub};

const DEBUG: bool = false;

/// Represents the tile type (pipe), named after their possible connection
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
enum TileType {
    /// '.'
    Ground,
    /// '|'
    NorthSouth,
    /// '-'
    EastWest, 
    /// 'L'
    NorthEast,
    /// 'J'
    NorthWest,
    /// '7'
    SouthWest,
    /// 'F'
    SouthEast,
    /// '?'
    Unknown
}

impl TileType {

    /// Returns the tile char representing the pipe
    fn get_char(&self) -> char {
        match self {
            &Self::Ground => { '.' }
            &Self::NorthSouth => { '|' }
            &Self::EastWest => { '-' }
            &Self::NorthEast => { 'L' }
            &Self::NorthWest => { 'J' }
            &Self::SouthWest => { '7' }
            &Self::SouthEast => { 'F' }
            &Self::Unknown => { '?' }
        }
    }

    /// Returns the direction output vector depeding on the tile type and the direction input vector
    fn get_pipe_redirection(&self, direction: &Direction) -> Result<Coords, String> {
        match self {
            &Self::Ground => { return Err(format!("No connection allowed to a {:?} tile...", self)); }
            &Self::NorthSouth => {
                match direction {
                    &Direction::North => { return Ok(Coords::new(0, -1)); }
                    &Direction::South => { return Ok(Coords::new(0, 1)); }
                    &Direction::West => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::East => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                }
            }
            &Self::EastWest => {
                match direction {
                    &Direction::North => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::South => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::West => { return Ok(Coords::new(-1, 0)); }
                    &Direction::East => { return Ok(Coords::new(1, 0)); }
                }
            }
            &Self::NorthEast => {
                match direction {
                    &Direction::North => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::South => { return Ok(Coords::new(1, 0)); }
                    &Direction::West => { return Ok(Coords::new(0, -1)); }
                    &Direction::East => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                }
            }
            &Self::NorthWest => {
                match direction {
                    &Direction::North => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::South => { return Ok(Coords::new(-1, 0)); }
                    &Direction::West => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::East => { return Ok(Coords::new(0, -1)); }
                }
            }
            &Self::SouthWest => {
                match direction {
                    &Direction::North => { return Ok(Coords::new(-1, 0)); }
                    &Direction::South => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::West => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::East => { return Ok(Coords::new(0, 1)); }
                }
            }
            &Self::SouthEast => {
                match direction {
                    &Direction::North => { return Ok(Coords::new(1, 0)); }
                    &Direction::South => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                    &Direction::West => { return Ok(Coords::new(0, 1)); }
                    &Direction::East => { return Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)); }
                }
            }
            &Self::Unknown => { return Err(format!("No connection allowed to a {:?} tile...", self)); }
        }
    }
}

/// Represents a direction, its the direction pointed by the vector
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
enum Direction {
    /// '^'
    North,
    /// 'v'
    South,
    /// '<'
    West,
    /// '>'
    East
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64
}

impl Coords {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Tile {
    /// (0, 0) located top left corner of the map
    coords: Coords,
    tile_type: TileType,
    is_start: bool // S
}

impl Tile {
    fn new(coords: Coords, tile_type: TileType, is_start: bool) -> Self {
        Self {
            coords,
            tile_type,
            is_start
        }
    }

    /// Returns the tile char representing the pipe
    fn get_char(&self) -> char {
        self.tile_type.get_char()
    }

    /// Returns the next coords based on the last tile and the current
    fn get_next_coords(&self, last_tile: &Tile) -> Result<Coords, String> {
        let diff_coords = self.coords - last_tile.coords;
        let direction: Direction;
        // >
        if diff_coords == Coords::new(1, 0) {
            direction = Direction::East;
        }
        // <
        else if diff_coords == Coords::new(-1, 0) {
            direction = Direction::West;
        }
        // v
        else if diff_coords == Coords::new(0, 1) {
            direction = Direction::South;
        }
        // ^
        else if diff_coords == Coords::new(0, -1) {
            direction = Direction::North;
        }
        // ?
        else {
            let err_msg = format!("[Error while getting computing the input direction (diff_coords: {:?})]", diff_coords);
            return Err(err_msg);
        }

        match self.tile_type.get_pipe_redirection(&direction) {
            Ok(val) => {
                return Ok(self.coords + val);
            }
            Err(error) => {
                let err_msg = format!("[Error while getting the pipe redirection: {error}]");
                return Err(err_msg);
            }
        }
    }
}

fn get_input(filename: &str) -> Result<(HashMap<Coords, Tile>, Coords), String> {
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


    // Parse the content into a hashmap
    let mut hm_tiles = HashMap::<Coords, Tile>::new();
    let mut start_coords = Coords::new(0, 0);
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for line in content.lines() {
        x = 0;
        for tile_char in line.chars() {
            match tile_char {
                '.' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::Ground, false)); }
                '|' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthSouth, false)); }
                '-' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::EastWest, false)); }
                'L' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthEast, false)); }
                'J' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthWest, false)); }
                '7' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::SouthWest, false)); }
                'F' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::SouthEast, false)); }
                'S' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::Unknown, true)); start_coords = Coords::new(x, y); }
                _   => { return Err(format!("Unknown tile char: '{tile_char}'")); }
            }
            x += 1;
        }
        y += 1;
    }

    match init_start_tile(&mut hm_tiles, &start_coords) {
        Ok(_) => {}
        Err(error) => {
            let err_msg = format!("[Error while initializing the starting tile: {error}]");
            return Err(err_msg);
        }
    }
    
    Ok((hm_tiles, start_coords))
}

fn display_tiles(hm_tiles: &HashMap<Coords, Tile>, max_x: &i64, max_y: &i64) -> () {
    for y in 0..max_y+1 {
        for x in 0..*max_x+1 {
            print!("{}", hm_tiles.get(&Coords::new(x, y)).unwrap().get_char());
        }
        println!();
    }
}

/// Initialize the start tile type based on its surroundings
fn init_start_tile(hm_tiles: &mut HashMap<Coords, Tile>, start_coords: &Coords) -> Result<(), String> {
    // Get surrouding tiles type (ignore the case of the start tile being on a border/corner, we unwrap all Option(Tile))
    let north_tile = hm_tiles.get(&(*start_coords - Coords::new(0, -1))).unwrap().clone();
    let north_connected: bool;
    let south_tile = hm_tiles.get(&(*start_coords - Coords::new(0, 1))).unwrap().clone();
    let south_connected: bool;
    let west_tile= hm_tiles.get(&(*start_coords - Coords::new(-1, 0))).unwrap().clone();
    let west_connected: bool;
    let east_tile= hm_tiles.get(&(*start_coords - Coords::new(1, 0))).unwrap().clone();
    let east_connected: bool;

    let start_tile = hm_tiles.get_mut(start_coords).unwrap();

    match north_tile.get_next_coords(start_tile) {
        Ok(_) => { north_connected = true }
        Err(_) => { north_connected = false }
    }
    match south_tile.get_next_coords(start_tile) {
        Ok(_) => { south_connected = true }
        Err(_) => { south_connected = false }
    }
    match west_tile.get_next_coords(start_tile) {
        Ok(_) => { west_connected = true }
        Err(_) => { west_connected = false }
    }
    match east_tile.get_next_coords(start_tile) {
        Ok(_) => { east_connected = true }
        Err(_) => { east_connected = false }
    }

    let start_tile_type: TileType;

    if north_connected && south_connected { start_tile_type = TileType::NorthSouth; }
    else if north_connected && east_connected { start_tile_type = TileType::SouthWest; }
    else if north_connected && west_connected { start_tile_type = TileType::SouthEast; }
    else if south_connected && east_connected { start_tile_type = TileType::NorthWest; }
    else if south_connected && west_connected { start_tile_type = TileType::NorthEast; }
    else if east_connected && west_connected { start_tile_type = TileType::EastWest; }
    else { return Err(format!("Couldn't initialize the Start tile: (north: {north_connected}, south: {south_connected}, west: {west_connected}, east: {east_connected}, ")); }

    println!("Successfully initialized Start tile to {:?}", start_tile_type);
    start_tile.tile_type = start_tile_type;

    Ok(())
}


/// Follows pipes from starting coords, and returns the nb step required to loop
fn follow_pipes(hm_tiles: &HashMap<Coords, Tile>, start_coords: &Coords, start_direction: &Direction) -> Result<u64, String> {
    //  Result variables
    let mut steps: u64 = 0;

    // Run variables
    let mut current_tile = hm_tiles.get(start_coords).unwrap();
    let mut last_tile = hm_tiles.get(&(*start_coords - current_tile.tile_type.get_pipe_redirection(start_direction).unwrap())).unwrap();

    while current_tile.coords != *start_coords || steps == 0 {
        match current_tile.get_next_coords(last_tile) {
            Ok(val) => {
                last_tile = current_tile;
                current_tile = hm_tiles.get(&val).unwrap();
                steps += 1;
            }
            Err(error) => {
                let err_msg = format!("[Error while initializing the starting tile: {error}]");
                return Err(err_msg);
            }
        }
    }

    Ok(steps)
}

fn main() -> Result<(), i8>{
    let filename: &str;
    let max_x: i64;
    let max_y: i64;
    if DEBUG {
        filename = "input_debug.txt";
        max_x = 19;
        max_y = 9;
    }
    else {
        filename = "input.txt";
        max_x = 139;
        max_y = 139;
    }

    let start_coords: Coords;
    let hm_tiles: HashMap::<Coords, Tile>;
    match get_input(filename) {
        Ok(val) => {
            hm_tiles = val.0;
            start_coords = val.1;
        }
        Err(error) => {
            eprintln!("Error while getting the input: Error: {error}");
            return Err(-1);
        }
    }

    // println!("hashmap: {:?}", hm_tiles);
    // println!("start_coords: {:?}", start_coords);

    // display_tiles(&hm_tiles, &max_x, &max_y);

    let result: u64;
    match follow_pipes(&hm_tiles, &start_coords, &Direction::North) {
        Ok(val) => {
            result = val;
        }
        Err(error) => {
            eprintln!("Error while following pipe: Error: {error}");
            return Err(-1);
        }
    }

    println!("result: {}", result/2);

    Ok(())
}
