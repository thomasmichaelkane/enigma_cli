#[derive(Debug)]
pub struct Rotor {
  // Define the state of the Enigma machine
  permutation: Vec<char>,
  offset: usize,
  length: usize,
  alphabet: Vec<char>,
}

impl Rotor {
  pub fn new(perm_str: String) -> Self {
    Rotor {
      permutation: perm_str.chars().collect::<Vec<char>>(),
      offset: 0,
      length: perm_str.len(),
      alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>(),
    }
  }
  
  pub fn get_offset(&self) -> usize {
    // Return the current offset
    self.offset
  }

  pub fn advance(&mut self) -> bool {
    // Move the offset to the next position, wrapping around when at the end
    self.offset = (self.offset + 1) % self.length;
    self.offset == 0
  }

  pub fn forward_permutation(&self, input_char: char) -> char {
    // Apply the rotors permutation in the forward direction
    self.apply_permutation(
      input_char, 
      &self.alphabet, 
      &self.permutation)
  }

  pub fn reverse_permutation(&self, input_char: char) -> char {
    // Apply the rotors permutation in the reverse direction
    self.apply_permutation(
      input_char, 
      &self.permutation, 
      &self.alphabet)
  }

  pub fn apply_permutation(&self, input_char: char, input_seq: &Vec<char>, output_seq: &Vec<char>) -> char {
    // Apply the permutation

    // Permutation logic
    let in_index = self.alphabet.iter().position(|l| l == &input_char).unwrap();
    let perm_index = (in_index + self.offset) % self.length;
    let perm_char = output_seq[perm_index];
    let out_index = input_seq.iter().position(|l| l == &perm_char).unwrap();

    // Wraparound logic for negative values
    let mut final_index = out_index as isize - self.offset as isize;
    if final_index < 0 {
        final_index += self.length as isize;  // If negative, wrap around
    }

    // Output character
    let out_char = self.alphabet[final_index as usize];
    return out_char;
  }
}
