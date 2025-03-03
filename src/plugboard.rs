use crate::ALPHABET;

const PERM_LEN: usize = ALPHABET.len();

#[derive(Debug)]
pub struct Plugboard {
  connections: [Option<char>; PERM_LEN],
  num_connections: usize,
}

impl Plugboard {
  
  pub fn new() -> Self {
    Plugboard {
      connections: [None; PERM_LEN],
      num_connections: 0,
    }
  }

  pub fn get_connections(&self) -> &[Option<char>; PERM_LEN] {
    &self.connections
  }

  pub fn get_num_connections(&self) -> usize {
    self.num_connections
  }

  pub fn add_connection(&mut self, c1: char, c2: char) {
    let i1 = ALPHABET.iter().position(|l| l == &c1).unwrap();
    let i2 = ALPHABET.iter().position(|l| l == &c2).unwrap();
    
    self.connections[i1] = Some(c2);
    self.connections[i2] = Some(c1);

    self.num_connections += 1;
    }

  pub fn permutation(&mut self, in_c: char) -> char {
    let i = ALPHABET.iter().position(|l| l == &in_c).unwrap();

    match self.connections[i] {
      Some(out_c) => out_c,
      None => in_c
    }
  }

  pub fn reset(&mut self) {
    self.connections = [None; PERM_LEN];
    self.num_connections = 0;
  }

}