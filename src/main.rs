mod model;
mod view;
mod utils;
mod rotor;
mod plugboard;

use std::env;

use view::EnigmaView;
use model::EnigmaModel;
use rotor::Rotor;
use plugboard::Plugboard;

use utils::{
    load_constants, 
    load_ascii_art, 
    load_ascii_mapping_top, 
    load_ascii_mapping_plugboard};

fn main() {

    // Parse args
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"-d".to_string());

    // Load ASCII art
    let frame = load_ascii_art()
        .expect("Failed to load ASCII art");
    
    // Load ASCII character mapping for the top view
    let ascii_mapping = load_ascii_mapping_top();

    // Load ASCII character mapping for the plugboard view
    let plugboard_mapping = load_ascii_mapping_plugboard();

    // Load constants
    let constants = load_constants()
        .expect("Failed to load YAML");

    // Create rotors
    let rotors: Vec<Rotor> = constants["rotor_permutations"]
        .as_vec()
        .expect("Expected rotor_permutations to be an array")
        .iter()
        .map(|perm| Rotor::new(perm
            .as_str()
            .expect("Expected string")
            .to_string()))
        .collect();

    // Create reflector
    let reflector: Option<Rotor> = Some(Rotor::new(constants["reflector_permutations"]
        .as_str()
        .expect("Expected string").to_string()));

    // Create plugboard
    let plugboard: Plugboard = Plugboard::new();
    
    // Create view
    let view = EnigmaView::new(
        frame,
        ascii_mapping,
        plugboard_mapping, 
        debug_mode);
    
    // Create model
    let mut enigma = EnigmaModel::new(
        view, 
        rotors, 
        reflector,
        plugboard,
        debug_mode); 

    // Wire plugboard
    enigma.wire_plugboard();

    // Run the enigma machine
    enigma.start_typing();

}