use std::error::Error;
use std::path::Path;
mod generators;
use generators::Generator;

fn available_generators(path: &Path) -> Vec<Box<dyn Generator>> {
  let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
  match extension {
    "rs" => vec![Box::new(generators::rust::Std {})],
    "rb" => vec![
      Box::new(generators::ruby::Std {}),
      Box::new(generators::ruby::Rspec {}),
    ],
    "py" => vec![Box::new(generators::python::Std {})],
    _ => vec![],
  }
}

pub fn run(root: &Path, path: &Path, test_runner: &str) -> Result<(), Box<dyn Error>> {
  let generators = available_generators(&path)
    .into_iter()
    .filter(|x| x.option_name() == test_runner)
    .collect::<Vec<Box<dyn Generator>>>();

  let generator = generators
    .first()
    .ok_or("No generators available for this file")?;

  generator.create_test(&root, &path)?;

  Ok(())
}
