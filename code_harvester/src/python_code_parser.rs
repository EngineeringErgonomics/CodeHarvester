use crate::code_structures::CodeStructure;
use regex::Regex;

/// Tokenizes the input Python code and returns a vector of tokens.
pub fn parse_python_code(input: &str) -> Vec<String> {
    let token_pattern = r#"[a-zA-Z_]\w*|\d+|\S"#;
    let re = Regex::new(token_pattern).unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Reads the content of a file, tokenizes the Python code, and returns a vector of tokens.
pub fn read_file_and_parse(file_path: &str) -> Vec<String> {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    // print parsed out
    println!("Python Code: {:?}", parse_python_code(&file_content));
    parse_python_code(&file_content)
}

/// Extracts code structures (classes and functions) from the input Python code and returns a vector of CodeStructure.
pub fn extract_code_structures(input: &str) -> Vec<CodeStructure> {
    let mut structures = Vec::new();
    let mut stack: Vec<(usize, String, Context)> = Vec::new();

    #[derive(Clone, PartialEq)]
    enum Context {
        Class,
        Function,
    }

    let mut lines = input.lines().collect::<Vec<_>>();
    lines.push(""); // push an empty line to handle the last code structure
    for (i, line) in lines.iter().enumerate() {
        let current_indent_level = line.chars().take_while(|c| *c == ' ').count();

        while let Some((indent, _, _)) = stack.last() {
            if *indent <= current_indent_level {
                break;
            }
            let (indent, structure, _context) = stack.pop().unwrap();
            if indent == 0 {
                structures.push(CodeStructure::from_code_string(&structure.trim_end()));
            } else if let Some((_, parent, _)) = stack.last_mut() {
                parent.push_str(&structure);
                parent.push('\n');
            }
        }

        if !line.trim().is_empty() {
            let mut line_to_push = line.to_string();
            let current_context = if line.trim().starts_with("class") {
                Context::Class
            } else if line.trim().starts_with("def") {
                Context::Function
            } else {
                stack
                    .last()
                    .map(|(_, _, context)| context.clone())
                    .unwrap_or_else(|| Context::Function)
            };

            // if the next line is not the last line and has greater indentation, append a newline to the current line
            if i < lines.len() - 1
                && lines[i + 1].chars().take_while(|c| *c == ' ').count() > current_indent_level
            {
                line_to_push.push('\n');
            }
            stack.push((current_indent_level, line_to_push, current_context));
        }
    }

    // Pop the remaining items in the stack
    while let Some((indent, structure, _)) = stack.pop() {
        if indent == 0 {
            structures.push(CodeStructure::from_code_string(structure.trim_end()));
        } else if let Some((_, parent, _)) = stack.last_mut() {
            parent.push_str(&structure);
            parent.push('\n');
        }
    }

    structures
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_parser_tokenize() {
        let input = "def foo():\n    return 42";
        let expected_tokens = vec!["def", "foo", "(", ")", ":", "return", "42"];

        let tokens = parse_python_code(input);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_read_file_and_parse() {
        // Create a temporary Python file for testing
        let temp_file_path = "temp_test_file.py";
        let temp_file_content = "def foo():\n    return 42";
        let mut temp_file = File::create(temp_file_path).unwrap();
        temp_file.write_all(temp_file_content.as_bytes()).unwrap();

        // Function to read the contents of a file and pass it to the parse_python_code function
        fn read_file_and_parse(file_path: &str) -> Vec<String> {
            let file_content = std::fs::read_to_string(file_path).unwrap();
            parse_python_code(&file_content)
        }

        let expected_tokens = vec!["def", "foo", "(", ")", ":", "return", "42"];

        let tokens = read_file_and_parse(temp_file_path);
        assert_eq!(tokens, expected_tokens);

        // Clean up the temporary file
        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_read_python_file_and_parse() {
        let sample_file_path = "code_harvester/samples/sample.py";

        let expected_tokens = vec![
            "def", "foo", "(", ")", ":", "return", "42", "class", "Bar", ":", "def", "__init__",
            "(", "self", ",", "x", ")", ":", "self", ".", "x", "=", "x", "def", "get_x", "(",
            "self", ")", ":", "return", "self", ".", "x",
        ];

        let tokens = read_file_and_parse(sample_file_path);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_extract_code_structures() {
        let code = "def foo():\n    return 42\n\nclass Bar:\n    def __init__(self, x):\n        self.x = x\n\n    def get_x(self):\n        return self.x\n";
        let mut actual = extract_code_structures(code);
        let mut expected = vec![
            CodeStructure::Function("def foo():\n    return 42".to_string()),
            CodeStructure::Class("class Bar:\n    def __init__(self, x):\n        self.x = x\n\n    def get_x(self):\n        return self.x".to_string()),
        ];

        actual.sort();
        expected.sort();

        println!("Actual: {:?}", actual);
        println!("Expected: {:?}", expected);

        assert_eq!(actual, expected);
    }
}
