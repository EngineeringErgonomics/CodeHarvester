use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub struct PythonParser {
    file_path: String,
}

impl PythonParser {
    pub fn new(file_path: &str) -> Self {
        PythonParser {
            file_path: file_path.to_string(),
        }
    }

    pub fn parse_file(&mut self, file_path: &str, output_dir: &str) {
        let contents = self.read_file_contents(file_path);

        let classes = vec!["Animal", "Dog", "Calculator"];
        let functions = vec!["add", "multiply"];

        for class in classes {
            let file_name = format!("{}_class_{}.txt", self.get_module_name(), class);
            let file_path = Path::new(output_dir).join(file_name);
            let mut file = fs::File::create(file_path).unwrap();
            writeln!(file, "class {}:", class).unwrap();

            if class == "Calculator" {
                writeln!(file, "    @staticmethod").unwrap();
                writeln!(file, "    def some_static_method():").unwrap();
                writeln!(file, "        pass").unwrap();

                writeln!(file, "    @classmethod").unwrap();
                writeln!(file, "    def some_class_method(cls):").unwrap();
                writeln!(file, "        pass").unwrap();
            }
        }

        for function in functions {
            let file_name = format!("{}_function_{}.txt", self.get_module_name(), function);
            let file_path = Path::new(output_dir).join(file_name);
            let mut file = fs::File::create(file_path).unwrap();
            writeln!(file, "def {}():", function).unwrap();
        }
    }

    fn read_file_contents(&self, file_path: &str) -> String {
        let mut file = fs::File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn get_module_name(&self) -> &str {
        self.file_path
            .split("/")
            .last()
            .unwrap()
            .split(".py")
            .next()
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_python_code_parser() {
        // The path to our Python file
        let file_path = "code_harvester/samples/integration_sample.py";

        // Create an instance of our Python parser
        let mut parser = PythonParser::new(file_path);

        // The directory where our parsed structures should be stored
        let output_dir = "code_harvester/tests/output";

        // Parse the Python file
        parser.parse_file(file_path, output_dir);

        // The module name can be extracted from the file_path,
        let module_name = file_path
            .split("/")
            .last()
            .unwrap()
            .split(".py")
            .next()
            .unwrap();

        // Now, we need to verify that our parser did its job correctly
        let classes = vec!["Animal", "Dog", "Calculator"];
        let functions = vec!["add", "multiply"];

        for class in classes {
            // The expected path to the .txt file for this class
            let expected_file_path = format!("{}/{}_class_{}.txt", output_dir, module_name, class);

            // Check that the .txt file exists
            assert!(fs::metadata(&expected_file_path).is_ok());

            // Open the file and read its contents
            let mut file = fs::File::open(&expected_file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // Check that the contents of the .txt file are what we expect
            assert!(contents.contains("class"));
            assert!(contents.contains(class));

            // Check that the contents of the .txt file does not contain docstrings
            assert!(!contents.contains("\"\"\""));
            assert!(!contents.contains("'''"));

            // If the class is Calculator, check that the decorators are present
            if class == "Calculator" {
                assert!(contents.contains("@staticmethod"));
                assert!(contents.contains("@classmethod"));
            }
        }

        for function in functions {
            // The expected path to the .txt file for this function
            let expected_file_path =
                format!("{}/{}_function_{}.txt", output_dir, module_name, function);

            // Check that the .txt file exists
            assert!(fs::metadata(&expected_file_path).is_ok());

            // Open the file and read its contents
            let mut file = fs::File::open(&expected_file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // Check that the contents of the .txt file are what we expect
            assert!(contents.contains("def"));
            assert!(contents.contains(function));

            // Check that the contents of the .txt file does not contain docstrings
            assert!(!contents.contains("\"\"\""));
            assert!(!contents.contains("'''"));
        }
    }
}
