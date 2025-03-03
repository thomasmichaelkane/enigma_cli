use crate::ALPHABET;

const PERM_LEN: usize = ALPHABET.len();

#[derive(Debug)]
pub struct Rotor {
  permutation: [char; PERM_LEN],
  offset: usize,
  length: usize,
}

impl Rotor {

  pub fn new(perm_str: &str) -> Self {
    
    let permutation = perm_str.chars()
                                          .map(|c| c.to_ascii_uppercase())
                                          .collect::<Vec<char>>()
                                          .try_into()
                                          .unwrap();
    
    Rotor {
      permutation,
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

  pub fn apply_permutation(&self, input_char: char, input_seq: &[char; PERM_LEN], output_seq: &[char; PERM_LEN]) -> char {
    // Apply the permutation

    // Find position of input character in the alphabet
    let in_index = ALPHABET.iter()
                                .position(|l| l == &input_char)
                                .unwrap();
    
    // Apply offset and wrap around
    let perm_index = (in_index + self.offset) % self.length;
    
    // Get character from output sequence
    let perm_char = output_seq[perm_index];
    
    // Find position of this character in input sequence
    let out_index = input_seq.iter()
                                .position(|l| l == &perm_char)
                                .unwrap();

    // Wraparound logic for negative values
    let mut final_index = out_index as isize - self.offset as isize;
    if final_index < 0 {
        final_index += self.length as isize;  // If negative, wrap around
    }

    // Return the final character from alphabet
    ALPHABET[final_index as usize]
  }
}
