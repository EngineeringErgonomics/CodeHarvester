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
