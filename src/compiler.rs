use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Compiler {
    Gcc,
    Gpp,
    Clang,
    Other(String),
}

impl FromStr for Compiler {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "gcc" => Ok(Compiler::Gcc),
            "g++" => Ok(Compiler::Gpp),
            "clang" => Ok(Compiler::Clang),
            other => Ok(Compiler::Other(other.to_string())),
        }
    }
}

impl std::fmt::Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Compiler::Gcc => write!(f, "gcc"),
            Compiler::Gpp => write!(f, "g++"),
            Compiler::Clang => write!(f, "clang"),
            Compiler::Other(name) => write!(f, "{}", name),
        }
    }
}
