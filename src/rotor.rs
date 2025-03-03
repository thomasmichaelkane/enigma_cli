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
      length: PERM_LEN,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_advance() {
    let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
    assert_eq!(rotor.offset, 0);
    
    // Test advancing and getting false (not a full rotation)
    assert_eq!(rotor.advance(), false);
    assert_eq!(rotor.offset, 1);
    
    // Set to almost wrapped around
    rotor.offset = 25;
    
    // Test advancing and getting true (full rotation)
    assert_eq!(rotor.advance(), true);
    assert_eq!(rotor.offset, 0);
  }

  #[test]
    fn test_forward_permutation() {
      let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
      assert_eq!(rotor.forward_permutation('A'), 'E');
      assert_eq!(rotor.forward_permutation('Z'), 'J');
    }

    #[test]
    fn test_reverse_permutation() {
      let rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
      assert_eq!(rotor.reverse_permutation('K'), 'B');
      assert_eq!(rotor.reverse_permutation('C'), 'Y');
    }

    #[test]
    fn test_rotation_affects_permutation() {
      let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
      
      // At offset 0
      let initial_mapping = rotor.forward_permutation('A');
      
      // Advance the rotor
      rotor.advance();
      
      // At offset 1, the mapping should be different
      let new_mapping = rotor.forward_permutation('A');
      assert_ne!(initial_mapping, new_mapping);
    }
    
    #[test]
    fn test_full_rotation_cycle() {
      let mut rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
      let initial_mapping = rotor.forward_permutation('A');
      
      // Rotate through a full cycle (26 positions)
      for _ in 0..26 {
          rotor.advance();
      }
      
      // After a full rotation, the mapping should be the same
      let final_mapping = rotor.forward_permutation('A');
      assert_eq!(initial_mapping, final_mapping);
    }
}