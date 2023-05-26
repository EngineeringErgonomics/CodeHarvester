mod tests {
    use code_harvester::parse_python_code;
    use code_harvester::read_file_and_parse;
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

    use code_harvester::code_structures::CodeStructure;
    use code_harvester::python_code_parser::extract_code_structures;

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
