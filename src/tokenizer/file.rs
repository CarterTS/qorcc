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

    /// Construct a token iterator
    pub fn iter<'a>(&'a self) -> FileTokenIterator<'a>
    {
        FileTokenIterator { i: 0, file: self }
    }

    /// Display an error message like interface for the given location
    pub fn display_arrow(&self, location: &Location, length: usize)
    {
        let start_line = location.line.max(3) - 3;
        let stop_line = location.line - 1;

        let lines = self.raw_text.lines().collect::<Vec<_>>();
        for i in start_line..=stop_line
        {
            println!("{:5}  {}", format!("{}", i + 1), lines[i]);
        }

        print!("      ");

        for _ in 0..location.column
        {
            print!(" ");
        }

        for _ in 0..length
        {
            print!("^");
        }

        println!();
    }
}

/// File Token Iterator
pub struct FileTokenIterator<'a>
{
    i: usize,
    file: &'a FileManager
}

impl<'a> std::iter::Iterator for FileTokenIterator<'a>
{
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.file.tokens.as_ref().map(|vector| {self.i += 1; vector.get(self.i - 1) }).flatten()
    }
}