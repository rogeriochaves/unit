use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate cmd_lib;

pub struct JUnit();

impl Generator for JUnit {
  fn option_name(&self) -> &'static str {
    "junit"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    String::from("mvn test")
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    let pom_xml_path = root.join("pom.xml");
    let mut pom_xml = fs::read_to_string(&pom_xml_path).or(Err("pom.xml not found"))?;

    if !pom_xml.contains("<dependencies>") {
      pom_xml = pom_xml.replace(
        "</project>",
        "    <dependencies>\n    </dependencies>\n</project>",
      );
    }
    if !pom_xml.contains("org.junit") {
      pom_xml = pom_xml.replace(
        "<dependencies>",
        r#"<dependencies>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter-api</artifactId>
            <version>RELEASE</version>
            <scope>test</scope>
        </dependency>"#,
      );
      fs::write(&pom_xml_path, pom_xml)?;
    }

    Ok(())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    self.setup(root)?;

    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let child_main_path = child_path
      .strip_prefix("main/")
      .or(Err("Source code needs to be inside src/main/"))?;
    let test_folder = root.join("src").join("test").join(child_main_path);
    let test_file_name = format!("{}Test.java", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    let java_package = child_main_path
      .to_str()
      .unwrap()
      .replace("/", ".")
      .replace("java.", "");
    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"package {};

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class {}Test {{
    @Test
    public void testItWorks() {{
        assertEquals(2, 1 + 1);
    }}
}}
"#,
        java_package, file_stem,
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
