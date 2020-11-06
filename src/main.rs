use std::fs::File;
use std::io::prelude::*;

fn main() {
  write();
}

fn read() {
  let mut file = File::open("info.txt").expect("Can't open file");

  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Ops...");

  println!("File Contents:\n\n {}", contents);
}

fn write() {
  let mut file = File::create("output.txt")
      .expect("Could not create file");
  
  file.write_all(b"Hello world")
      .expect("Cannot write to the file, sorry mate.");
}