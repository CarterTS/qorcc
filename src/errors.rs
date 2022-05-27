#![allow(dead_code)]

use crate::{preprocessor::PreprocessorError, compiler::Compiler};
pub enum CompilerError
{
    BadFilename(String),
    PreprocessorError(PreprocessorError)
}

impl std::fmt::Display for CompilerError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            CompilerError::BadFilename(name) => write!(f, "Unable to open file {}", name),
            CompilerError::PreprocessorError(error) => write!(f, "Preprocessor error: {}", error)
        }
    }
}

impl CompilerError
{
    pub fn output_more(self, compiler: &mut Compiler)
    {
        match self
        {
            CompilerError::PreprocessorError(error) =>
            {
                if let Ok(file) = compiler.get_file_manager(&error.location.filename)
                {
                    file.display_arrow(&error.location, error.arrow_length);
                }
            },
            _ => {}
        }
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;