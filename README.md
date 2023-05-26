# CodeHarvester
## Getting Started

Welcome to CodeHarvester, a language-agnostic library designed to efficiently extract meaningful chunks of code from source code projects. This capability facilitates sophisticated analysis and provides contextual understanding for machine learning models.

### Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

Clone the CodeHarvester repository to your local machine:

```bash
git clone https://github.com/your-username/CodeHarvester.git
cd CodeHarvester
```

### Build

To build the project, navigate to the project directory and use Cargo, Rust's package manager.

```bash
cargo build
```

This command creates an executable file in `target/debug/`.

### Usage

In the CodeHarvester project, we provide several key functions to extract meaningful code from Python source code.

#### Parse Python Code

This function tokenizes the input Python code and returns a vector of tokens. 

```rust
use code_harvester::parse_python_code;

let code = "def add(x, y):\n    return x + y";
let tokens = parse_python_code(&code);
```

#### Read File and Parse

This function reads a Python file, tokenizes the code, and returns a vector of tokens. 

```rust
use code_harvester::read_file_and_parse;

let tokens = read_file_and_parse("/path/to/your/python/file.py");
```

#### Extract Code Structures

This function extracts code structures (classes and functions) from the input Python code and returns a vector of `CodeStructure`. 

```rust
use code_harvester::extract_code_structures;

let code = "def add(x, y):\n    return x + y";
let structures = extract_code_structures(&code);
```

## Running Tests

To run all tests:

```bash
cargo test
```

### Contributing

Please read `CONTRIBUTING.md` for the process for submitting pull requests to us.

### License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

### Questions?

Feel free to create an issue for any questions or problems you might have. We'll gladly help you to make the most out of CodeHarvester.

### Happy coding!

## Roadmap

As CodeHarvester continues to evolve, we are committed to expanding our functionality. Here's a brief glimpse into our roadmap:

1. **Parsing Python**: We are currently in the midst of enhancing our capabilities for parsing Python code structures. This includes comprehensive parsing of Python classes, functions, decorators, and more.

2. **Expanding to Rust**: Once we have strengthened our Python parsing, we plan to extend our solution to include the Rust language. This will allow for analysis and understanding of projects built with this increasingly popular system-level language.

3. **JavaScript & TypeScript**: Further down the road, we intend to implement functionality for parsing JavaScript and TypeScript. This will extend CodeHarvester's usability to front-end and full-stack projects, broadening its potential applications and usefulness.

Our roadmap reflects our dedication to continual improvement and user-centric design. We welcome feedback and contributions that help us move closer to our goals.