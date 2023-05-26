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

    pub fn get_content(&self) -> &str {
        match self {
            CodeStructure::Function(s) => s,
            CodeStructure::Class(s) => s,
        }
    }

    pub fn get_name(&self) -> String {
    let content = self.get_content();
    let keyword_end = content.find(|c: char| c.is_whitespace()).unwrap();
    let name_start = content[keyword_end..]
        .find(|c: char| c.is_alphabetic())
        .unwrap()
        + keyword_end;
    let name_end = content[name_start..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap();
    content[name_start..name_start + name_end].to_string()
}


    pub fn get_type(&self) -> &str {
        match self {
            CodeStructure::Function(_) => "function",
            CodeStructure::Class(_) => "class",
        }
    }
}
