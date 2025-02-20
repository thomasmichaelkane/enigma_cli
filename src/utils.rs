use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
use yaml_rust::{Yaml, YamlLoader};
use crossterm::style::Color;

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 
    'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
];

pub fn load_constants() -> Result<Yaml, Box<dyn Error>> {
    // Load the constants.yaml file and parse it into a YAML structure

    let constants_path = Path::new("constants.yaml");
    let yaml_str = fs::read_to_string(constants_path)?;
    let docs = YamlLoader::load_from_str(&yaml_str)?;
    
    docs.into_iter().next().ok_or_else(|| "Empty YAML file".into())
}

pub fn load_ascii_art() -> Result<Vec<Vec<(char, Option<Color>)>>, Box<dyn Error>> {
    // Load ascii art from "ascii.txt" file

    let ascii_path = Path::new("assets/ascii.txt");
    
    let ascii_art: Vec<Vec<(char, Option<Color>)>> = fs::read_to_string(ascii_path)?
        .lines()
        .map(|line| line
            .chars()
            .map(|c| (c, None))
            .collect())
        .collect();

    Ok(ascii_art)
}

pub fn load_ascii_mapping_top() -> HashMap<char, (usize, usize)> {
    // Create map of ASCII characters that need to be located
    
    // Rotor and keyboard origin coordinates
    let (r0x, r0y): (usize, usize) = (5, 14);
    let (k0x, k0y): (usize, usize) = (9, 10);
    
    HashMap::from([

        // Rotor relative locations
        ('1', (0 + r0x, 0 + r0y)),
        ('2', (0 + r0x, 7 + r0y)),
        ('3', (0 + r0x, 14 + r0y)),
        
        // Lamp relative locations
        ('Q', (0 + k0x, 0 + k0y)),
        ('W', (0 + k0x, 4 + k0y)),
        ('E', (0 + k0x, 8 + k0y)),
        ('R', (0 + k0x, 12 + k0y)),
        ('T', (0 + k0x, 16 + k0y)),
        ('Z', (0 + k0x, 20 + k0y)),
        ('U', (0 + k0x, 24 + k0y)),
        ('I', (0 + k0x, 28 + k0y)),
        ('O', (0 + k0x, 32 + k0y)),
        ('A', (1 + k0x, 2 + k0y)),
        ('S', (1 + k0x, 6 + k0y)),
        ('D', (1 + k0x, 10 + k0y)),
        ('F', (1 + k0x, 14 + k0y)),
        ('G', (1 + k0x, 18 + k0y)),
        ('H', (1 + k0x, 22 + k0y)),
        ('J', (1 + k0x, 26 + k0y)),
        ('K', (1 + k0x, 30 + k0y)),
        ('P', (2 + k0x, 0 + k0y)),
        ('Y', (2 + k0x, 4 + k0y)),
        ('X', (2 + k0x, 8 + k0y)),
        ('C', (2 + k0x, 12 + k0y)),
        ('V', (2 + k0x, 16 + k0y)),
        ('B', (2 + k0x, 20 + k0y)),
        ('N', (2 + k0x, 24 + k0y)),
        ('M', (2 + k0x, 28 + k0y)),
        ('L', (2 + k0x, 32 + k0y)),
        
        // Keyboard relative locations
        ('q', (4 + k0x, 0 + k0y)),
        ('w', (4 + k0x, 4 + k0y)),
        ('e', (4 + k0x, 8 + k0y)),
        ('r', (4 + k0x, 12 + k0y)),
        ('t', (4 + k0x, 16 + k0y)),
        ('z', (4 + k0x, 20 + k0y)),
        ('u', (4 + k0x, 24 + k0y)),
        ('i', (4 + k0x, 28 + k0y)),
        ('o', (4 + k0x, 32 + k0y)),
        ('a', (5 + k0x, 2 + k0y)),
        ('s', (5 + k0x, 6 + k0y)),
        ('d', (5 + k0x, 10 + k0y)),
        ('f', (5 + k0x, 14 + k0y)),
        ('g', (5 + k0x, 18 + k0y)),
        ('h', (5 + k0x, 22 + k0y)),
        ('j', (5 + k0x, 26 + k0y)),
        ('k', (5 + k0x, 30 + k0y)),
        ('p', (6 + k0x, 0 + k0y)),
        ('y', (6 + k0x, 4 + k0y)),
        ('x', (6 + k0x, 8 + k0y)),
        ('c', (6 + k0x, 12 + k0y)),
        ('v', (6 + k0x, 16 + k0y)),
        ('b', (6 + k0x, 20 + k0y)),
        ('n', (6 + k0x, 24 + k0y)),
        ('m', (6 + k0x, 28 + k0y)),
        ('l', (6 + k0x, 32 + k0y)),
    ])
}

pub fn load_ascii_mapping_plugboard() -> HashMap<char, (usize, usize)> {
    // Create map of ASCII characters that need to be located
    
    // Plugboard origin coordinates
    let (p0x, p0y): (usize, usize) = (18, 10);
    HashMap::from([

        // Plugboard relative locations
        ('Q', (0 + p0x, 0 + p0y)),
        ('W', (0 + p0x, 4 + p0y)),
        ('E', (0 + p0x, 8 + p0y)),
        ('R', (0 + p0x, 12 + p0y)),
        ('T', (0 + p0x, 16 + p0y)),
        ('Z', (0 + p0x, 20 + p0y)),
        ('U', (0 + p0x, 24 + p0y)),
        ('I', (0 + p0x, 28 + p0y)),
        ('O', (0 + p0x, 32 + p0y)),
        ('A', (2 + p0x, 2 + p0y)),
        ('S', (2 + p0x, 6 + p0y)),
        ('D', (2 + p0x, 10 + p0y)),
        ('F', (2 + p0x, 14 + p0y)),
        ('G', (2 + p0x, 18 + p0y)),
        ('H', (2 + p0x, 22 + p0y)),
        ('J', (2 + p0x, 26 + p0y)),
        ('K', (2 + p0x, 30 + p0y)),
        ('P', (4 + p0x, 0 + p0y)),
        ('Y', (4 + p0x, 4 + p0y)),
        ('X', (4 + p0x, 8 + p0y)),
        ('C', (4 + p0x, 12 + p0y)),
        ('V', (4 + p0x, 16 + p0y)),
        ('B', (4 + p0x, 20 + p0y)),
        ('N', (4 + p0x, 24 + p0y)),
        ('M', (4 + p0x, 28 + p0y)),
        ('L', (4 + p0x, 32 + p0y)),
    ])
}