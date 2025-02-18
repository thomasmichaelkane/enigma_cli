use std::fs;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEventKind};
use event::{poll, read};
use event::Event;

use crate::rotor::Rotor;
use crate::view::EnigmaView;

pub struct EnigmaModel {
  // Define the state of the Enigma machine
  view: EnigmaView,
  rotors: Vec<Rotor>,
  reflector: Option<Rotor>,
  alphabet: Vec<char>,
  debug: bool,
}

impl EnigmaModel {

  // Implement methods to manipulate the Enigma machine's state
  pub fn new(view: EnigmaView, 
             rotors: Vec<Rotor>,
             reflector: Option<Rotor>,
             debug: bool) -> Self {
    EnigmaModel {
      view,
      rotors,
      reflector,
      alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>(),
      debug,
    }
  }

  pub fn run(&mut self) {
    // Run the enigma machine

    // Start view
    self.view.start();

    // Main loop to process key presses and update the enigma model
    loop {
    
      // Listen for input
      if let Ok(true) = poll(Duration::from_millis(100)) {
        if let Ok(Event::Key(key_event)) = read() {
          
          // Handle key press
          if key_event.kind == KeyEventKind::Press {
            match key_event.code {

              // Character detected
              KeyCode::Char(c) => {

                // Convert to uppercase
                let c = c.to_ascii_uppercase();
                if self.debug {println!("received key press: {}", c)};
                
                match c {
                  // Rotor number match -> manual rotation
                  '1' | '2' | '3' => {self.manual_rotate(c);},

                  // Any other character -> auto rotate and key press logic
                  _ => {if self.alphabet.contains(&c) {
                          self.auto_rotate();
                          self.key_press(c);
                        }},
                }
              }

              // Escape key -> exit
              KeyCode::Esc => break,

              // Enter key -> save encrypted message
              KeyCode::Enter => self.save_message(),
              
              // Other keys -> ignore
              _ => {}
            }
          }
        }
      }
    }

    // End view
    self.view.end();

  }

  fn get_rotor_letter(&self, rotor_num: usize) -> char {
    // Return the current letter of the specified rotor
    self.alphabet[self.rotors[rotor_num].get_offset()]
  }

  fn key_press(&mut self, mut c: char) {
  // Update the enigma model on keypress

    // Update the keyboard view at character C
    self.view.update_keyboard(c.to_ascii_lowercase());
    if self.debug {print!("{} (START)", c)};

    // Pass C through the rotors in the forward direction
    for i in (0..self.rotors.len()).rev() {
      c = self.rotors[i].forward_permutation(c);
      if self.debug {print!("-> {} ({})", c, i)};
    }

    // Apply the reflector to C if present
    if let Some(reflector) = &self.reflector {
      c = reflector.forward_permutation(c);
      if self.debug {print!("-> {} (REFLECT)", c)};
    }

    // Pass C through the rotors in the reverse direction
    for i in 0..self.rotors.len() as usize {
      c = self.rotors[i].reverse_permutation(c);
      if self.debug {print!("-> {} ({})", c, i)};
    }

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
