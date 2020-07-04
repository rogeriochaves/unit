use std::env;
use std::error::Error;
use std::path::Path;

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
  unit [FILE]
  unit <SUBCOMMAND>

SUBCOMMANDS:
  setup create all the necessary bootstrap for running tests on your project
"#
    );
    std::process::exit(0);
  }
  unit::run(Path::new(""), Path::new(filename))?;

  Ok(())
}
