use std::collections::HashMap;

use super::codegen;
use super::parser;
use super::preprocessor;
use super::tokenizer::*;
use super::errors::*;
use super::settings::CompilerSettings;

pub struct Compiler<'a>
{
    settings: &'a CompilerSettings,
    active_filenames: Vec<String>,
    loaded_files: HashMap<String, FileManager>
}

impl<'a> Compiler<'a>
{
    pub fn with_settings(settings: &'a CompilerSettings) -> Self
    {
        Self
        {
            settings,
            active_filenames: Vec::new(),
            loaded_files: HashMap::new()
        }
    }

    /// Ensure that a particular file is loaded
    pub fn load_file(&mut self, filename: &str) -> CompilerResult<()>
    {
        if !self.loaded_files.contains_key(filename)
        {
            self.active_filenames.push(String::from(filename));
            let manager = FileManager::open_new(self.active_filenames.last().unwrap())?;

            self.loaded_files.insert(String::from(filename), manager);
        }

        Ok(())
    }

    /// Get the file manager for a file or return an error if it can't be read
    pub fn get_file_manager(&mut self, filename: &str) -> CompilerResult<&mut FileManager>
    {
        self.load_file(filename)?;

        if let Some(fm) = self.loaded_files.get_mut(filename)
        {
            Ok(fm)
        }
        else
        {
            unreachable!()
        }
    }

    pub fn compile(&mut self, filename: &str) -> CompilerResult<()>
    {
        trace!("Compiling file {}", filename);

        // Open the file, or return a BadFilename error
        let file = self.get_file_manager(filename)?;
        
        // Tokenize the file
        file.tokenize()?;

        for token in file.tokens.as_ref().unwrap()
        {
            println!("{}", token);
        }
        
        Ok(())
    }
}