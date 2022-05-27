use std::io::Read;

use crate::errors::{CompilerResult, CompilerError};

use super::Location;
use super::Token;

#[derive(Debug, Clone)]
pub struct FileManager
{
    pub filename: String,
    pub raw_text: String,
    pub tokens: Option<Vec<Token>>
}

impl FileManager
{
    /// Open a new file manager
    pub fn open_new(filename: &str) -> CompilerResult<Self>
    {
        let filename = String::from(filename);
        let mut file = std::fs::File::open(&filename).map_err(|_| CompilerError::BadFilename(String::from(&filename)))?;
        let mut raw_text = String::new();

        file.read_to_string(&mut raw_text).expect("Unable to read from file");

        Ok(FileManager
        {
            filename,
            raw_text,
            tokens: None
        })
    }

    /// Construct a location for the current file
    pub fn location(&self, line: usize, column: usize) -> Location
    {
        Location::construct(&self.filename, line, column)
    }

    /// Tokenize the given file
    pub fn tokenize(&mut self) -> CompilerResult<()>
    {
        self.tokens = Some(super::tokenize(&self)?);

        Ok(())
    }
}