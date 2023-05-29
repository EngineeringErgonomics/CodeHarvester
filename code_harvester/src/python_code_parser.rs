use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct PythonParser {
    current_line: String,
    reader: BufReader<File>,
}

impl PythonParser {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        PythonParser {
            current_line: String::new(),
            reader,
        }
    }

    pub fn parse_file(&mut self, file_path: &str, output_dir: &str) {
        while let Ok(_) = self.read_line() {
            if self.current_line.contains("class") {
                self.parse_class(output_dir);
            } else if self.current_line.contains("def") {
                self.parse_function(output_dir);
            }
        }
    }

    fn read_line(&mut self) -> Result<(), std::io::Error> {
    self.current_line.clear(); // ensure the line is clear before reading new content
    match self.reader.read_line(&mut self.current_line) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                // no bytes read, indicating end of file
                Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "End of file reached",
                ))
            } else {
                // bytes were read, line updated
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}


    fn parse_class(&mut self, output_dir: &str) {
        // implement parsing of a class here
    }

    fn parse_function(&mut self, output_dir: &str) {
        // implement parsing of a function here
    }

    // additional parse methods for other structures (if, while, for, etc.) can go here

    fn write_to_file(&self, file_path: &str, content: &str) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
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
