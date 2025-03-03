use::std::fs;

pub struct Message {
  content: String,
  formatted: String,
}

impl Message {
  pub fn new() -> Self {
    Message {
      content: String::new(),
      formatted: String::new()
    }      
  }

  pub fn add(&mut self, c: char) {
    // Add C to the message and go to new line if necessary
    if (self.content.len() % 54) == 0 {self.content.push_str("\n")};
    self.content.push(c);
  }
  
  pub fn clear(&mut self) {
    self.content.clear();
  }

  pub fn read(&self) -> &str {
    &self.content
  }

  pub fn print(&mut self) {
    self.format();
    fs::create_dir_all("print").expect("Unable to create print dir");
    fs::write("print/msg.txt", self.formatted.clone()).expect("Unable to write file");
  }

  fn format(&mut self) {
    // Format the message into enigma style
    self.formatted = self.content
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