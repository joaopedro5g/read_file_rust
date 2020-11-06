use std::fs:File;
use std::io::prelude::*;

fn main() {
  let file = File::create("output.txt")
      .expect("Could not create file");
  
  file.write_all(b"Hello world Rust")
      .expect("Cannot write to the file, sorry mate.");
}