use crate::utils::ALPHABET;

#[derive(Debug)]
pub struct Rotor {

  permutation: [char; 26],
  offset: usize,
  length: usize,
}

impl Rotor {

  pub fn new(perm_str: String) -> Self {
    Rotor {
      permutation: perm_str.chars().collect::<Vec<char>>().try_into().unwrap(),
      offset: 0,
      length: perm_str.len(),
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
      &ALPHABET, 
      &self.permutation)
  }

  pub fn reverse_permutation(&self, input_char: char) -> char {
    // Apply the rotors permutation in the reverse direction
    self.apply_permutation(
      input_char, 
      &self.permutation, 
      &ALPHABET)
  }

  pub fn apply_permutation(&self, input_char: char, input_seq: &[char; 26], output_seq: &[char; 26]) -> char {
    // Apply the permutation

    // Permutation logic
    let in_index = ALPHABET.iter().position(|l| l == &input_char).unwrap();
    let perm_index = (in_index + self.offset) % self.length;
    let perm_char = output_seq[perm_index];
    let out_index = input_seq.iter().position(|l| l == &perm_char).unwrap();

    // Wraparound logic for negative values
    let mut final_index = out_index as isize - self.offset as isize;
    if final_index < 0 {
        final_index += self.length as isize;  // If negative, wrap around
    }

    // Output character
    let out_char = ALPHABET[final_index as usize];
    return out_char;
  }
}
