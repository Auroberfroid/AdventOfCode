use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::ops::{Add, Sub};
use ansi_term::Colour;
use crossterm::{cursor, ExecutableCommand};


const DEBUG: bool = true;

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

    /// Returns the direction output vector depending on the tile type and the direction input vector
    fn get_pipe_redirection(&self, direction: &Direction) -> Result<Coords, String> {
        match self {
            &Self::Ground => { Err(format!("No connection allowed from a {:?} tile...", self)) }
            &Self::NorthSouth => {
                match direction {
                    &Direction::North => { Ok(Coords::new(0, -1)) }
                    &Direction::South => { Ok(Coords::new(0, 1)) }
                    &Direction::West => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::East => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                }
            }
            &Self::EastWest => {
                match direction {
                    &Direction::North => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::South => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::West => { Ok(Coords::new(-1, 0)) }
                    &Direction::East => { Ok(Coords::new(1, 0)) }
                }
            }
            &Self::NorthEast => {
                match direction {
                    &Direction::North => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::South => { Ok(Coords::new(1, 0)) }
                    &Direction::West => { Ok(Coords::new(0, -1)) }
                    &Direction::East => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                }
            }
            &Self::NorthWest => {
                match direction {
                    &Direction::North => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::South => { Ok(Coords::new(-1, 0)) }
                    &Direction::West => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::East => { Ok(Coords::new(0, -1)) }
                }
            }
            &Self::SouthWest => {
                match direction {
                    &Direction::North => { Ok(Coords::new(-1, 0)) }
                    &Direction::South => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::West => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::East => { Ok(Coords::new(0, 1)) }
                }
            }
            &Self::SouthEast => {
                match direction {
                    &Direction::North => { Ok(Coords::new(1, 0)) }
                    &Direction::South => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                    &Direction::West => { Ok(Coords::new(0, 1)) }
                    &Direction::East => { Err(format!("No connection allowed from {:?} to a {:?} tile...", direction, self)) }
                }
            }
            &Self::Unknown => { Err(format!("No connection allowed from a {:?} tile...", self)) }
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
enum TileStatus {
    /// Is the starting tile
    Start,
    /// Is part of the loop
    Loop,
    /// Is contained by the loop
    In,
    /// Is NOT contained by the loop
    Out,
    /// Undefined status 
    Undefined
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Tile {
    /// (0, 0) located top left corner of the map
    coords: Coords,
    /// The type of the tile
    tile_type: TileType,
    /// The status of the tile [Start, Loop, In, Out, Undefined]
    status: TileStatus
}

impl Tile {
    fn new(coords: Coords, tile_type: TileType, status: TileStatus) -> Self {
        Self {
            coords,
            tile_type,
            status
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

/// Represents the squeezed rat between 2 pipes movig within the tilemap
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct SqueezedRat {
    coords: [Coords; 2]
}

impl SqueezedRat {
    fn new(coords: [Coords; 2]) -> Result<Self, String> {
        if ![Coords::new(1, 0), Coords::new(-1, 0), Coords::new(0, 1), Coords::new(0, -1)].contains(&(*coords.get(0).unwrap() - *coords.get(1).unwrap())) {
            return Err("A squeezed rat can only live between 2 adjacents pipes!".to_string());
        }
        Ok(Self {
            coords
        })
    }
        
    /// Returns a the coord 0 - coord 1 
    fn get_detla_coords(&self) -> Coords {
        *self.coords.get(0).unwrap() - *self.coords.get(1).unwrap()
    }

    /// Returns directions a squeezed rat can head toward to (only 2)
    fn get_direction(&self) -> Result<[Direction; 2], String> {
        // Shouldn't need to wrap it in result, but we never know...
        if [Coords::new(1, 0), Coords::new(-1, 0)].contains(&self.get_detla_coords()) {
            Ok([Direction::West, Direction::East])
        }
        else if [Coords::new(0, 1), Coords::new(0, -1)].contains(&self.get_detla_coords()) {
            Ok([Direction::North, Direction::North])
        }
        else {
            Err(format!("Error while getting squeezed rat direction (coords.0: {:?} && coords.1: {:?})", *self.coords.get(0).unwrap(), *self.coords.get(0).unwrap()))
        }
    }

}

fn get_input(filename: &str) -> Result<(HashMap<Coords, Tile>, Coords, i64, i64), String> {
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
                '.' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::Ground, TileStatus::Undefined)); }
                '|' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthSouth,  TileStatus::Undefined)); }
                '-' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::EastWest,  TileStatus::Undefined)); }
                'L' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthEast,  TileStatus::Undefined)); }
                'J' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::NorthWest,  TileStatus::Undefined)); }
                '7' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::SouthWest,  TileStatus::Undefined)); }
                'F' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::SouthEast,  TileStatus::Undefined)); }
                'S' => { hm_tiles.insert(Coords::new(x, y), Tile::new(Coords::new(x, y), TileType::Unknown,  TileStatus::Start)); start_coords = Coords::new(x, y); }
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
    
    Ok((hm_tiles, start_coords, x, y))
}

fn display_tiles_inplace(hm_tiles: &HashMap<Coords, Tile>, max_x: &i64, max_y: &i64, mut file_to_write: &std::io::Stdout) -> () {
    let mut tile: &Tile;
    let mut screen_content = String::new();
    for y in 0..max_y+1 {
        for x in 0..*max_x+1 {
            tile = hm_tiles.get(&Coords::new(x, y)).unwrap();
            match tile.status {
                TileStatus::Start => {
                    screen_content += &format!("{}", Colour::Purple.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::Loop => {
                    screen_content += &format!("{}", Colour::Red.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::In => {
                    screen_content += &format!("{}", Colour::Green.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::Out => {
                    screen_content += &format!("{}", Colour::Cyan.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                _ => {
                    screen_content += &format!("{}", hm_tiles.get(&Coords::new(x, y)).unwrap().get_char());
                }
            }
        }
        screen_content += "\n";
    }

    write!(file_to_write, "{}", screen_content).unwrap();
    file_to_write.execute(cursor::MoveUp((*(max_y) + 1) as u16)).unwrap();
}

fn display_tiles(hm_tiles: &HashMap<Coords, Tile>, max_x: &i64, max_y: &i64) -> () {
    let mut tile: &Tile;
    // ((<max_x + 1> char + '\n' per line) * <max_y + 1> lines) + ANSI chars (~factor 3).... -> allow faster display because less realloc
    let mut screen_content = String::with_capacity((3 * (max_x + 2) * (max_y + 1)) as usize);
    for y in 0..max_y+1 {
        for x in 0..*max_x+1 {
            tile = hm_tiles.get(&Coords::new(x, y)).unwrap();
            match tile.status {
                TileStatus::Start => {
                    screen_content += &format!("{}", Colour::Purple.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::Loop => {
                    screen_content += &format!("{}", Colour::Red.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::In => {
                    screen_content += &format!("{}", Colour::Green.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                TileStatus::Out => {
                    screen_content += &format!("{}", Colour::Cyan.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));
                }
                _ => {
                    screen_content += &format!("{}", hm_tiles.get(&Coords::new(x, y)).unwrap().get_char());
                }
            }
        }
        screen_content += "\n";
    }

    print!("{}", screen_content);
}

fn display_tiles_map_outer_loop_overlap(hm_tiles: &HashMap<Coords, Tile>, hm_loop_contours: &HashMap<Coords, Tile>, max_x: &i64, max_y: &i64) -> () {
    let mut hm_tile: &Tile;
    let mut hm_loop_contour: &Tile;
    let mut coords_key: Coords;
    // ((<max_x + 1> char + '\n' per line) * <max_y + 1> lines) + ANSI chars (~factor 3).... -> allow faster display because less realloc
    let mut screen_content = String::with_capacity((3 * (max_x + 2) * (max_y + 1)) as usize);   // let mut screen_content = Vec::<char>::with_capacity((3 * (max_x + 2) * (max_y + 1)) as usize);
    for y in 0..max_y+1 {
        for x in 0..*max_x+1 {
            coords_key = Coords::new(x, y);
            hm_tile = hm_tiles.get(&coords_key).unwrap();
            match hm_tile.status {
                TileStatus::Start => {
                    screen_content += &format!("{}", Colour::Purple.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));    // Colour::Green.paint(hm_tiles.get(&coords_key).unwrap().get_char().to_string()).to_string().chars().for_each(|x| screen_content.push(x));
                }
                TileStatus::Loop => {
                    hm_loop_contour = hm_loop_contours.get(&coords_key).unwrap();
                    match hm_loop_contour.status {
                        TileStatus::Loop => {
                            screen_content += &format!("{}", Colour::Red.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));    // Colour::Red.paint(hm_tiles.get(&coords_key).unwrap().get_char().to_string()).to_string().chars().for_each(|x| screen_content.push(x));
                        }
                        _ => {
                            screen_content += &format!("{}", Colour::RGB(160, 110, 5).paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));    // Colour::RGB(160, 110, 5).paint(hm_tiles.get(&coords_key).unwrap().get_char().to_string()).to_string().chars().for_each(|x| screen_content.push(x));
                        }
                    }
                }
                TileStatus::In => {
                    screen_content += &format!("{}", Colour::Green.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));    // Colour::Purple.paint(hm_tiles.get(&coords_key).unwrap().get_char().to_string()).to_string().chars().for_each(|x| screen_content.push(x));
                }
                TileStatus::Out => {
                    screen_content += &format!("{}", Colour::Cyan.paint(hm_tiles.get(&Coords::new(x, y)).unwrap().get_char().to_string()));    // Colour::Cyan.paint(hm_tiles.get(&coords_key).unwrap().get_char().to_string()).to_string().chars().for_each(|x| screen_content.push(x));
                }
                _ => {
                    screen_content += &format!("{}", hm_tiles.get(&Coords::new(x, y)).unwrap().get_char());     // screen_content.push(hm_tiles.get(&coords_key).unwrap().get_char());
                }
            }
        }
        screen_content.push('\n');
    }

    print!("{}", screen_content); // let screen_content_string: String = screen_content.into_iter().collect();
}

/// Initialize the start tile type based on its surroundings
fn init_start_tile(hm_tiles: &mut HashMap<Coords, Tile>, start_coords: &Coords) -> Result<(), String> {
    // Get surrouding tiles type (we ignore the case of the start tile being on a border/corner... such sneaky case, even for AOC)
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
fn follow_pipes(hm_tiles: &mut HashMap<Coords, Tile>, start_coords: &Coords, display: bool, max_x: i64, max_y: i64) -> Result<u64, String> {
    //  Result variables
    let mut steps: u64 = 0;

    // Run variables
    let mut current_tile = hm_tiles.get(start_coords).unwrap().clone();
    let mut last_tile: Tile;
    // Can't get enough ident :)
    match current_tile.tile_type.get_pipe_redirection(&Direction::North) {
        Ok(val) => { last_tile = hm_tiles.get(&(*start_coords + val)).unwrap().clone(); }
        Err(_) => {
            match current_tile.tile_type.get_pipe_redirection(&Direction::East) {
                Ok(val) => { last_tile = hm_tiles.get(&(*start_coords + val)).unwrap().clone(); }
                Err(_) => {
                    match current_tile.tile_type.get_pipe_redirection(&Direction::South) {
                        Ok(val) => { last_tile = hm_tiles.get(&(*start_coords + val)).unwrap().clone(); }
                        Err(_) => {
                            match current_tile.tile_type.get_pipe_redirection(&Direction::West) {
                                Ok(val) => { last_tile = hm_tiles.get(&(*start_coords + val)).unwrap().clone(); }
                                Err(_) => {
                                    return Err("Couldn't find any direction allowing to initiate the follow pipe".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    while current_tile.coords != *start_coords || steps == 0 {
        match current_tile.get_next_coords(&last_tile) {
            Ok(val) => {
                last_tile = current_tile;
                let mut_tile = hm_tiles.get_mut(&val).unwrap();
                if mut_tile.status != TileStatus::Start {
                    mut_tile.status = TileStatus::Loop;
                }
                current_tile = *hm_tiles.get(&val).unwrap();
                steps += 1;
            }
            Err(error) => {
                let err_msg = format!("[Error while getting next tile coords (steps: {steps},  last_tile: {:?}, current_tile: {:?}): {error}]", last_tile, current_tile);
                return Err(err_msg);
            }
        }
        if display {
            display_tiles_inplace(hm_tiles, &max_x, &max_y, &stdout());
        }
    }

    Ok(steps)
}

/// Returns a 'gross' loop contouring
fn get_loop_contouring(hm_tiles: &HashMap<Coords, Tile>, max_x: &i64, max_y: &i64, display: bool, display_overlapping: bool) -> HashMap<Coords, Tile> {
    let mut loop_contouring = hm_tiles.clone();
    // Clear the loop_contouring from its tilestatus (except start)
    for x in 0..*max_x+1 {
        for y in 0..*max_y {
            let mut_tile = loop_contouring.get_mut(&Coords::new(x, y)).unwrap();
            if mut_tile.status != TileStatus::Start {
                mut_tile.status = TileStatus::Undefined;
            }
        }
    }

    // Process horizontal contouring
    for x in 0..*max_x+1 {
        let mut last_y_in_loop: i64 = 0;
        let mut loop_tile_in_row = false;
        for y in 0..*max_y {
            let current_key = &Coords::new(x, y);
            match hm_tiles.get(current_key).unwrap().status {
                TileStatus::Loop => {
                    if last_y_in_loop == 0 {
                        loop_contouring.get_mut(current_key).unwrap().status = TileStatus::Loop;
                    }
                    last_y_in_loop = y;
                    loop_tile_in_row = true;
                }
                _ => {
                    continue;
                }
            }
        }
        if loop_tile_in_row {
            loop_contouring.get_mut(&Coords::new(x, last_y_in_loop)).unwrap().status = TileStatus::Loop;
        }
    }

    // Process vertical contouring
    for y in 0..*max_y {
        let mut last_x_in_loop: i64 = 0;
        let mut loop_tile_in_col = false;
        for x in 0..*max_x+1 {
            let current_key = &Coords::new(x, y);
            match hm_tiles.get(current_key).unwrap().status {
                TileStatus::Loop => {
                    if last_x_in_loop == 0 {
                        loop_contouring.get_mut(current_key).unwrap().status = TileStatus::Loop;
                    }
                    last_x_in_loop = x;
                    loop_tile_in_col = true;
                }
                _ => {
                    continue;
                }
            }
        }
        if loop_tile_in_col {
            loop_contouring.get_mut(&Coords::new(last_x_in_loop, y)).unwrap().status = TileStatus::Loop;
        }
    }
    
    if display {
        display_tiles(&loop_contouring, max_x, max_y);
    }

    if display_overlapping {
        display_tiles_map_outer_loop_overlap(hm_tiles, &loop_contouring, max_x, max_y);
    }

    loop_contouring
}

/// Set the status of the tiles that are sure to be out to Out (out of the loop)
fn define_sure_out(hm_tiles: &mut HashMap<Coords, Tile>, max_x: &i64, max_y: &i64, display: bool, display_overlapping: bool) -> HashMap<Coords, Tile> {
    let mut loop_contouring = get_loop_contouring(hm_tiles, max_x, max_y, display, display_overlapping);
    
    let mut coords_key: Coords;
    // Process vertical
    for x in 0..*max_x+1 {
        let mut min_y_loop:i64 = 0;
        let mut max_y_loop:i64 = 0;
        let mut y_loop_crossed = false;
        for y in 0..*max_y+1 {
            coords_key = Coords::new(x, y);
            match loop_contouring.get(&coords_key).unwrap().status {
                TileStatus::Loop => {
                    if !y_loop_crossed {
                        min_y_loop = y;
                        y_loop_crossed = true;
                    }
                    else {
                        max_y_loop = y;
                    }
                }
                _ => { continue; }
            }
        }
        for y in 0..*max_y+1 {
            coords_key = Coords::new(x, y);
            if y < min_y_loop || y > max_y_loop {
                hm_tiles.get_mut(&coords_key).unwrap().status = TileStatus::Out;
            }
            else if min_y_loop == max_y_loop && min_y_loop == 0 {
                hm_tiles.get_mut(&coords_key).unwrap().status = TileStatus::Out;
            }
        }
    }

    // Process horizontal
    for y in 0..*max_y {
        let mut min_x_loop:i64 = 0;
        let mut max_x_loop:i64 = 0;
        let mut x_loop_crossed = false;
        for x in 0..*max_x+1 {
            coords_key = Coords::new(x, y);
            match loop_contouring.get(&coords_key).unwrap().status {
                TileStatus::Loop => {
                    if !x_loop_crossed {
                        min_x_loop = x;
                        x_loop_crossed = true;
                    }
                    else {
                        max_x_loop = x;
                    }
                }
                _ => { continue; }
            }
        }
        for x in 0..*max_x+1 {
            coords_key = Coords::new(x, y);
            if x < min_x_loop || x > max_x_loop {
                hm_tiles.get_mut(&coords_key).unwrap().status = TileStatus::Out;
            }
            else if min_x_loop == max_x_loop && min_x_loop == 0 {
                hm_tiles.get_mut(&coords_key).unwrap().status = TileStatus::Out;
            }
        }
    }

    // Expand the Out status to every directly connected tile that is not Loop
    for _ in 0..2 { // The twice expansion is arbitrary based on observations on the result... not so clean but works for my input, might need to be adjusted for other cases
        for x in 0..*max_x+1 {
            for y in 0..*max_y+1 {
                coords_key = Coords::new(x, y);
                match hm_tiles.get(&coords_key).unwrap().status {
                    TileStatus::Undefined => {
                        let mut expanded = false;
                        for coord in [&(coords_key - Coords::new(-1, 0)), 
                                               &(coords_key - Coords::new(1, 0)),
                                               &(coords_key - Coords::new(0, -1)),
                                               &(coords_key - Coords::new(0, 1))] {
                            if ! expanded {
                                match hm_tiles.get(coord) {
                                    Some(val_tile) => {
                                        match val_tile.status {
                                            TileStatus::Out => {
                                                hm_tiles.get_mut(&coords_key).unwrap().status = TileStatus::Out;
                                                expanded = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Now that we have a complete picture of Out that touches borders, we can complete our 'gross' loop_contouring
    // We check for transition from Out to Loop or reverse
    let mut current_tile_status: TileStatus;
    for x in 0..*max_x+1 {
        let mut last_tile_status = TileStatus::Out; // Reasonable init value since all left and top borders a Out (in my input at least)
        for y in 0..*max_y+1 {
            coords_key = Coords::new(x, y);
            current_tile_status = hm_tiles.get(&coords_key).unwrap().status;
            if current_tile_status == TileStatus::Loop && last_tile_status == TileStatus::Out {
                loop_contouring.get_mut(&coords_key).unwrap().status = TileStatus::Loop;
            }
            else if current_tile_status == TileStatus::Out && last_tile_status == TileStatus::Loop {
                loop_contouring.get_mut(&(Coords::new(x, y - 1))).unwrap().status = TileStatus::Loop;
            }
            last_tile_status = current_tile_status;
        }
    }
    
    for y in 0..*max_y+1 {
        let mut last_tile_status = TileStatus::Out;
        for x in 0..*max_x+1 {
            coords_key = Coords::new(x, y);
            current_tile_status = hm_tiles.get(&coords_key).unwrap().status;
            if current_tile_status == TileStatus::Loop && last_tile_status == TileStatus::Out {
                loop_contouring.get_mut(&coords_key).unwrap().status = TileStatus::Loop;
            }
            else if current_tile_status == TileStatus::Out && last_tile_status == TileStatus::Loop {
                loop_contouring.get_mut(&(Coords::new(x - 1, y))).unwrap().status = TileStatus::Loop;
            }
            last_tile_status = current_tile_status;
        }
    }

    loop_contouring
}


fn main() -> Result<(), i8>{
    let filename: &str;
    let max_x: i64;
    let max_y: i64;
    if DEBUG {
        filename = "input_debug.txt";
    }
    else {
        filename = "input.txt";
    }

    let start_coords: Coords;
    let mut hm_tiles: HashMap::<Coords, Tile>;
    match get_input(filename) {
        Ok(val) => {
            hm_tiles = val.0;
            start_coords = val.1;
            max_x = val.2 - 1;
            max_y = val.3 - 1;
    }
        Err(error) => {
            eprintln!("Error while getting the input: Error: {error}");
            return Err(-1);
        }
    }
    
    match follow_pipes(&mut hm_tiles, &start_coords, false, max_x, max_y) {
        Ok(_) => { println!("Sucessfully followed the pipes!"); }
        Err(error) => {
            eprintln!("Error while following pipe: Error: {error}");
            return Err(-1);
        }
    }
    
    // println!("Loop: ");
    // display_tiles(&hm_tiles, &max_x, &max_y);
    // println!("\n");
    // println!("Loop Contouring: ");
    // get_loop_contouring(&hm_tiles, &max_x, &max_y, false, true);

    let loop_contouring = define_sure_out(&mut hm_tiles, &max_x, &max_y, false, false);
    // display_tiles(&hm_tiles, &max_x, &max_y);
    display_tiles_map_outer_loop_overlap(&hm_tiles, &loop_contouring, &max_x, &max_y);

    Ok(())
}
