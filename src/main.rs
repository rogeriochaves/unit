use std::env;
use std::error::Error;
use std::path::Path;
extern crate simple_error;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let filename = args
    .get(1)
    .ok_or("Provide filename to create the test file for, type --help for more info")?;

  if filename == "--help" {
    println!(
      r#"
Universal Test Generator (unit)
setup and create tests for any programming language

USAGE:
  unit [FILE] [OPTIONS]

OPTIONS:
  --available shows all test runners generators available for that file
"#
    );
    std::process::exit(0);
  }

  let generator = args
    .get(2)
    .map(|x| x.replace("--", ""))
    .unwrap_or(String::from("std"));

  let current_dir = env::current_dir()?;
  unit::run(&current_dir, Path::new(filename), &generator)?;

  Ok(())
}
