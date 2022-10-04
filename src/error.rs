use std::fmt::{Display, Formatter};
use std::process::exit;

#[derive(Debug)]
pub struct EmuError {
    reason: String,
    file_name: String,
    line: u32,
}

impl EmuError {
    pub fn new(reason: String, file_name: String, line: u32) -> Self {
        Self {
            reason,
            file_name,
            line,
        }
    }
}

impl Display for EmuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "There are some error {}", self.reason);
        writeln!(f, "Found in {}:{}", self.file_name, self.line)
    }
}

pub fn handle_result<T>(result: Result<T, EmuError>) -> T {
    match result {
        Ok(res) => res,
        Err(error) => {
            println!("{:#}", error);
            exit(0);
        }
    }
}
