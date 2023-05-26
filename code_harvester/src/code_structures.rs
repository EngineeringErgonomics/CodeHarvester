use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(PartialEq, Debug, Eq)]
pub enum CodeStructure {
    Function(String),
    Class(String),
    // Add other code structure types as needed
}

use std::cmp::{Ord, PartialOrd};

impl Ord for CodeStructure {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for CodeStructure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for CodeStructure {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            CodeStructure::Function(s) => write!(f, "{}", s),
            CodeStructure::Class(s) => write!(f, "{}", s),
        }
    }
}

impl CodeStructure {
    pub fn from_code_string(s: &str) -> Self {
        if s.starts_with("class ") {
            CodeStructure::Class(s.to_string())
        } else if s.starts_with("def ") {
            CodeStructure::Function(s.to_string())
        } else {
            panic!("Invalid code structure string");
        }
    }
}
