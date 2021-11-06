use std::error::Error;
use std::path::Path;
mod generators;
use generators::Generator;
use simple_error::bail;

fn available_generators(path: &Path) -> Vec<Box<dyn Generator>> {
  let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
  match extension {
    "rs" => vec![
      Box::new(generators::rust::Std {}),
      Box::new(generators::rust::Integration {}),
    ],
    "rb" => vec![
      Box::new(generators::ruby::Std {}),
      Box::new(generators::ruby::Rspec {}),
    ],
    "py" => vec![Box::new(generators::python::Std {})],
    "js" => vec![Box::new(generators::javascript::Jest {})],
    "pm" => vec![
      Box::new(generators::perl::Std {}),
      Box::new(generators::perl::TestSpec {}),
    ],
    "elm" => vec![Box::new(generators::elm::ElmTest {})],
    "clj" => vec![Box::new(generators::clojure::Std {})],
    "java" => vec![Box::new(generators::java::JUnit {})],
    "ex" => vec![Box::new(generators::elixir::Std {})],
    _ => vec![],
  }
}

pub fn run(root: &Path, path: &Path, test_runner: &str) -> Result<(), Box<dyn Error>> {
  let generators_for_file = available_generators(&path);
  if generators_for_file.is_empty() {
    bail!("No generators available for this file");
  }

  let generators_for_file_names = generators_for_file
    .iter()
    .map(|x| format!("--{}", x.option_name()))
    .collect::<Vec<String>>();

  if test_runner == "available" {
    println!(
      "Available generators: {}",
      generators_for_file_names.join(", ")
    );
    return Ok(());
  }

  let matching_generators = generators_for_file
    .iter()
    .filter(|x| x.option_name() == test_runner)
    .collect::<Vec<&Box<dyn Generator>>>();

  let mut generator = matching_generators.first().map(|x| *x);

  if generator.is_none() && test_runner == "std" {
    generator = generators_for_file.get(0);
  }

  match generator {
    None => {
      bail!(format!(
        "No {} generator found for this file, available generators: {}",
        test_runner,
        generators_for_file_names.join(", ")
      ));
    }
    Some(generator) => generator.create_test(&root, &path),
  }
}
