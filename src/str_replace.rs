// This code is from: https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/10

use std::fs::File;
use std::io;
use std::io::{prelude::*, Write};
use std::path::Path;

#[derive(Debug)]
pub struct StrReplace {
    data: String,
}

impl StrReplace {
    /// Creates StrReplace from the contents of a file at the given path
    pub fn from_file(path: &str) -> StrReplace {
        let filepath = Path::new(path);
        let mut file = File::open(filepath).unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data)
            .expect("Failed to read file.");

        StrReplace { data: data }
    }

    /// Replace the occurence of one string with another
    /// returns self for chainability.

    pub fn replace(&mut self, search: &str, replacement: &str) -> &mut Self {
        self.data = self.data.replace(search, replacement);
        self
    }

    /// Writes the possibly mutated data to a file at the given destination
    pub fn to_file(&self, dst: &str) -> Result<bool, io::Error> {
        match File::create(dst) {
            Ok(mut file) => match file.write_all(self.data.as_bytes()) {
                Ok(_) => Ok(true),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Makes a &str out of StrReplace for further use
    pub fn to_str(&self) -> &str {
        &self.data
    }
}
