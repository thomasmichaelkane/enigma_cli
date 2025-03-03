pub mod cli;
pub mod config;
pub mod model;
pub mod view;
pub mod loader;
pub mod rotor;
pub mod plugboard;
pub mod message;
pub mod validation;

use std::error::Error;

use config::Config;
use model::EnigmaModel;
use view::EnigmaView;
use rotor::Rotor;
use plugboard::Plugboard;

pub const MAX_PLUGS: usize = 10;

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 
    'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z'
];

pub fn run (config: Config)-> Result<(), Box<dyn Error>> {
  
  // Load ASCII art
  let frame = loader::ascii_art()?;

  // Load permutations
  let permutations = loader::permutations_yaml()?;

  // Load ASCII character mapping for the top view
  let ascii_mapping_top = loader::ascii_mapping_top();

  // Load ASCII character mapping for the plugboard view
  let ascii_mapping_plugboard = loader::ascii_mapping_plugboard();

  // Create rotors
  let rotors: Vec<Rotor> = permutations["rotor"]
    .as_vec()
    .expect("Expected rotor_permutations to be an array")
    .iter()
    .map(|perm| Rotor::new(perm
        .as_str()
        .expect("Expected string")))
    .collect();

  // Create reflector
  let reflector: Option<Rotor> = Some(Rotor::new(permutations["reflector"]
    .as_str()
    .expect("Expected string")));

  // Create plugboard
  let plugboard: Plugboard = Plugboard::new();

  // Create view
  let view = EnigmaView::new(
    frame,
    ascii_mapping_top,
    ascii_mapping_plugboard,
  );

  // Create model
  let mut enigma = EnigmaModel::new(
    view, 
    rotors, 
    reflector,
    plugboard,
    config,
  ); 

  // Wire plugboard
  enigma.wire_plugboard();

  // Run the enigma machine
  enigma.start_typing();

  Ok(())

}
