use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate cmd_lib;
use simple_error::bail;

pub struct JUnit();

impl Generator for JUnit {
  fn option_name(&self) -> &'static str {
    "junit"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    let pom_xml_path = Path::new("pom.xml");

    if pom_xml_path.exists() {
      return String::from("mvn test");
    }
    return String::from("gradle test");
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    let pom_xml_path = root.join("pom.xml");
    let build_gradle_path = root.join("build.gradle");

    if pom_xml_path.exists() {
      let mut pom_xml = fs::read_to_string(&pom_xml_path).or(Err("Could not read pom.xml"))?;

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
    } else if build_gradle_path.exists() {
      let mut build_gradle =
        fs::read_to_string(&build_gradle_path).or(Err("Could not read build.gradle"))?;
      if !build_gradle.contains("org.junit") {
        if !build_gradle.contains("dependencies {") {
          build_gradle = format!("{}\ndependencies {{\n}}\n", build_gradle);
        }
        build_gradle = build_gradle.replace(
          "dependencies {",
          r#"dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.+")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.+")"#,
        );
        if !build_gradle.contains("test {") {
          build_gradle = format!("{}\ntest {{\n}}\n", build_gradle);
        }
        if !build_gradle.contains("useJUnitPlatform") {
          build_gradle = build_gradle.replace("test {", "test {\n    useJUnitPlatform()");
        }
        fs::write(&build_gradle_path, build_gradle)?;
      }
    } else {
      bail!("Could not find pom.xml nor build.gradle, unit does not know how to setup dependencies for test runner in this project");
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
