use std::fs;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEventKind};
use event::{poll, read};
use event::Event;

use crate::rotor::Rotor;
use crate::plugboard::Plugboard;
use crate::view::EnigmaView;
use crate::utils::ALPHABET;

pub struct EnigmaModel {
  view: EnigmaView,
  rotors: Vec<Rotor>,
  reflector: Option<Rotor>,
  plugboard: Plugboard,
  debug: bool,
}

impl EnigmaModel {

  pub fn new(view: EnigmaView, 
             rotors: Vec<Rotor>,
             reflector: Option<Rotor>,
             plugboard: Plugboard,
             debug: bool) -> Self {
    EnigmaModel {
      view,
      rotors,
      reflector,
      plugboard,
      debug,
    }
  }

  pub fn start_typing(&mut self) {
    // Enter typing mode

    self.view.start("top");
    
    loop {
      if let Ok(true) = poll(Duration::from_millis(100)) {
        if let Ok(Event::Key(key_event)) = read() {
          
          if key_event.kind != KeyEventKind::Press {
            continue;
          }
          
          match key_event.code {
            KeyCode::Esc => break,
            KeyCode::Enter => self.save_message(),
            KeyCode::Char(c) => self.handle_character(c),
            _ => {}
          }
        }
      }
    }

    self.view.end();
  }

  pub fn wire_plugboard(&mut self) {
    // Enter pluboard wiring mode

    self.view.start("front");
    let mut initial_plug: Option<char> = None;
    
    loop {
      if let Ok(true) = poll(Duration::from_millis(100)) {
        if let Ok(Event::Key(key_event)) = read() {
          
          if key_event.kind != KeyEventKind::Press {
            continue;
          }

          if self.plugboard.get_num_connections() == 10 {
            break;
          }
          
          match key_event.code {
            KeyCode::Esc => {
              self.plugboard.reset();
              break;
            },
            KeyCode::Enter => {
              self.handle_plugboard_enter(&mut initial_plug);
              break;
            }
            KeyCode::Char(c) => {
              self.handle_plugboard_char(c, &mut initial_plug);
            },
            _ => {}
          }
        }
      }
    }

    self.view.end();
  }

  fn handle_plugboard_enter(&mut self, initial_plug: &mut Option<char>) {
    // Handle Enter KeyCode during wiring phase
    if let Some(c) = *initial_plug {
      self.view.remove_plug(c);
    }
  }

  fn handle_plugboard_char(&mut self, c: char, initial_plug: &mut Option<char>) {
    // Handle characters during wiring phase
    let c = c.to_ascii_uppercase();
    
    if self.is_already_plugged(c, *initial_plug) {
      return;
    }

    if self.debug {
      println!("received key press: {}", c);
    }

    match *initial_plug {
      None => self.start_new_connection(c, initial_plug),
      Some(ic) => self.complete_connection(ic, c, initial_plug),
    }
  }

  fn is_already_plugged(&self, c: char, initial_plug: Option<char>) -> bool {
    // Check if there is already a plug at that character
    self.plugboard.get_connections().contains(&Some(c)) || initial_plug == Some(c)
  }

  fn start_new_connection(&mut self, c: char, initial_plug: &mut Option<char>) {
    // Start a new plugboard connection and display
    *initial_plug = Some(c);
    self.view.add_initial_plug(c);
  }

  fn complete_connection(&mut self, initial_char: char, current_char: char, initial_plug: &mut Option<char>) {
    // Complete new plugboard connection and display
    self.view.add_final_plug(current_char, self.plugboard.get_num_connections());
    self.plugboard.add_connection(initial_char, current_char);
    *initial_plug = None;
    
    if self.debug {
      println!("Added plug connection: {}-{}", initial_char, current_char);
    }
  }

  fn handle_character(&mut self, c: char) {
    // Handle characters during typing phase
    let c = c.to_ascii_uppercase();
    
    if self.debug {
      println!("received key press: {}", c);
    }
    
    match c {
      '1' | '2' | '3' => self.manual_rotate(c),
      c if ALPHABET.contains(&c) => {
        self.auto_rotate();
        self.key_press(c);
      }
      _ => {}
    }
  }

  fn get_rotor_letter(&self, rotor_num: usize) -> char {
    // Return the current letter of the specified rotor
    ALPHABET[self.rotors[rotor_num].get_offset()]
  }

  fn key_press(&mut self, mut c: char) {
  // Update the enigma model on keypress

    // Update the keyboard view at character C
    self.view.update_keyboard(c.to_ascii_lowercase());
    if self.debug {print!("(IN) {} ", c)};

    // Pass C through the plugboard
    c = self.plugboard.permutation(c);
    if self.debug {print!("-> [PLUG] -> {} ", c)};

    // Pass C through the rotors in the forward direction
    for i in (0..self.rotors.len()).rev() {
      c = self.rotors[i].forward_permutation(c);
      if self.debug {print!("-> [R{}] -> {} ", i, c)};
    }

    // Apply the reflector to C if present
    if let Some(reflector) = &self.reflector {
      c = reflector.forward_permutation(c);
      if self.debug {print!("-> [REFLECT] -> {} ", c)};
    }

    // Pass C through the rotors in the reverse direction
    for i in 0..self.rotors.len() as usize {
      c = self.rotors[i].reverse_permutation(c);
      if self.debug {print!("-> [R{}] -> {} ", i, c)};
    }

    // Pass C through the plugboard
    c = self.plugboard.permutation(c);
    if self.debug {print!("-> [PLUG] -> {} (OUT)", c)};

    if self.debug {
      println!();
      println!("------");
    }

    // Update the lamp view at the new character C
    self.view.update_keyboard(c.to_ascii_uppercase());

  }

  fn manual_rotate(&mut self, rotor_char: char) {
    // Manually rotate the specified rotor
    let rotor_num = (rotor_char.to_digit(10).unwrap() as usize) - 1;
    self.rotate(rotor_num);

  }

  fn auto_rotate(&mut self) {
    // Autotmatically rotate rotors on key press
    for i in (0..self.rotors.len()).rev() {
      let full_rev = self.rotate(i);
      if !full_rev {break};
    }
  }

  fn rotate(&mut self, rotor_num: usize) -> bool {
    // Rotate the specified rotor and animate in view, return true if full revolution
    let curr_char = self.get_rotor_letter(rotor_num);
    let full_rev: bool = self.rotors[rotor_num].advance();
    let next_char = self.get_rotor_letter(rotor_num);
    let rotor_char = char::from_digit((rotor_num + 1) as u32, 10).unwrap();
    self.view.rotate_rotor(rotor_char, curr_char, next_char);
    
    if self.debug {
      println!("Rotor {} turned to: {}", rotor_num, next_char);
      println!("------");
    };

    return full_rev
  }

  fn save_message(&mut self) {
    // Save formated encrypted message to msg.txt
    let message = self.view.get_and_wipe_message();
    let formatted_message = self.format_message(message);
    fs::write("print/msg.txt", formatted_message).expect("Unable to write file");
  }

  fn format_message(&mut self, message: String) -> String {
    // Format the message into enigma style
    message
      .chars()
      .filter(|&c| c != '\n')
      .enumerate()
      .flat_map(|(i, c)| {
        let mut chunk = vec![c];

        if (i + 1) % 40 == 0 {
          chunk.push('\n');
        } else if (i + 1) % 5 == 0 {
          chunk.push(' ');
        }
        chunk
      })
      .collect()
  }

}
