use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;

use crossterm::{
  execute,
  style::{Color, Stylize},
  terminal::{self, ClearType},
  cursor::{self},
};

pub struct EnigmaView {
  frame: Vec<Vec<(char, Option<Color>)>>,
  ascii_mapping_top: HashMap<char, (usize, usize)>,
  ascii_mapping_plugboard: HashMap<char, (usize, usize)>,
  previous_key_press: Option<char>,
  previous_lamp: Option<char>,
  previous_wire: Option<char>,
  stdout: io::Stdout,
  key_color: Color,
  lamp_color: Color,
  front_view: bool,
  message_buffer: String,
}

impl EnigmaView {
  pub fn new(
    frame: Vec<Vec<(char, Option<Color>)>>,
    ascii_mapping_top: HashMap<char, (usize, usize)>,
    ascii_mapping_plugboard: HashMap<char, (usize, usize)>, 
  ) -> Self {
    EnigmaView { 
      frame,
      ascii_mapping_top,
      ascii_mapping_plugboard,
      previous_key_press: None,
      previous_lamp: None,
      previous_wire: None,
      stdout: io::stdout(),
      key_color: Color::Grey,
      lamp_color: Color::Yellow,
      front_view: false,
      message_buffer: String::new(),
             }
  }

  pub fn start(&mut self, perspective: &str) {
    // Start the view for the chosen perspective
    terminal::enable_raw_mode().unwrap();
    
    match perspective {
      "top" => self.front_view = false,
      "front" => self.front_view = true,
      _ => {}
    }

    self.flip();
  }
  
  pub fn end(&mut self) {
    // End view for the current perspective
    execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
    execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
    terminal::disable_raw_mode().unwrap();
  }

  pub fn add_initial_plug(&mut self, c: char) {
    // Mark initial plug position
    if let Some((x, y)) = self.ascii_mapping_plugboard.get(&c) {
      self.frame[*x][*y].0 = '+';
      self.frame[*x][*y].1 = Some(Color::Yellow);
      self.previous_wire = Some(c);
    }

    self.flip()
  }

  pub fn add_final_plug(&mut self, c: char, num_connection: usize) {
    // Complete the connection by labelling initial and final plugs
    let num_char = char::from_digit(num_connection as u32, 10).unwrap();

    // Final plug
    if let Some((x, y)) = self.ascii_mapping_plugboard.get(&c) {
      self.frame[*x][*y].0 = num_char;
      self.frame[*x][*y].1 = Some(Color::DarkGrey);
    }

    // Initial plug
    if let Some(pc) = self.previous_wire.take() {
      if let Some((px, py)) = self.ascii_mapping_plugboard.get(&pc) {
        self.frame[*px][*py].0 = num_char;
        self.frame[*px][*py].1 = Some(Color::DarkGrey);
      }
    }

    self.flip()
  }

  pub fn remove_plug(&mut self, c: char) {
    // Reset styling at the plug for specififec charcater
    if let Some((x, y)) = self.ascii_mapping_plugboard.get(&c) {
      self.frame[*x][*y].0 = ':';
      self.frame[*x][*y].1 = None;
    }
    self.flip()
  }
  
  pub fn flip(&mut self) {
    // Clear screen, move cursor to top-left, print new frame
    execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
    execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
    self.print_colored_frame();
  }

  pub fn update_keyboard(&mut self, c: char) {
    // Update the highlighting of keyboard and lamp
    // UPPERCASE: Lamp // LOWERCASE: Keyboard //

    // Get the correct color and previous key
    let color = if c.is_ascii_uppercase() { self.lamp_color } else { self.key_color };
    let previous = if c.is_ascii_uppercase() { &mut self.previous_lamp } else { &mut self.previous_key_press };

    // Set the new key color
    if let Some((x, y)) = self.ascii_mapping_top.get(&c) {
      self.frame[*x][*y].1 = Some(color);
    }

    // Reset the previous key color
    if let Some(pc) = previous.take() {
      if let Some((px, py)) = self.ascii_mapping_top.get(&pc) {
        self.frame[*px][*py].1 = None;
      }
    }

    // Store the newly pressed key
    *previous = Some(c);

    // If updating the lamp, flip the screen
    if c.is_ascii_uppercase() {
      self.flip();
    };

  }

  pub fn rotate_rotor(&mut self, rotor_c: char, curr_c: char, next_c: char) {
    // Update the corresponding rotor display with roll animation
    let (y, x) = self.ascii_mapping_top[&rotor_c];
    self.rotor_animate(curr_c, ' ', ' ', x, y);
    self.rotor_animate(' ', ' ', next_c, x, y);
    self.rotor_animate(' ', next_c, ' ', x, y);
    self.flip();
  }

  fn rotor_animate(&mut self, t_char: char, m_char: char, b_char: char, x: usize, y: usize) {
    // Rotor roll animation
    self.frame[y-1][x].0 = t_char;
    self.frame[y][x].0 = m_char;
    self.frame[y+1][x].0 = b_char;
    self.lag();
    self.flip();
  }

  fn lag(&self) {
    // Short lag for animation
    sleep(Duration::from_millis(75));
  }

  pub fn update_message_buffer(&mut self, message: &str) {
    // Update the message at the bottom of the screen
    self.message_buffer = String::from(message);
  }

  pub fn wipe_message_buffer(&mut self) {
    // Update the message at the bottom of the screen
    self.message_buffer.clear();
  }

  fn print_colored_frame(&mut self) {
    // Print the current frame with colour highlighting

    let frame = if self.front_view { &self.frame[16..] } else { &self.frame[..17] };
    
    // Create output buffer
    let mut output_buffer = String::new();

    // Iterate over the frame and apply color if necessary
    for row in frame {
      for &c in row {
        match c.1 {
          Some(color) => {
            let styled_char = format!("{}", c.0.with(Color::Black).on(color)); // Apply the color
            output_buffer.push_str(&styled_char);
          }
          None => {
            let styled_char = format!("{}", c.0); // No color
            output_buffer.push_str(&styled_char);
          }
        }
      }
      output_buffer.push_str("\r\n");
    }

    // Add the message at the bottom
    output_buffer.push_str("\r\n");
    output_buffer.push_str(&self.message_buffer);

    // Write everything at once
    write!(self.stdout, "{}", output_buffer).unwrap();
    self.stdout.flush().unwrap();

  }
}