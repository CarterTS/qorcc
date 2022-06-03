#![allow(dead_code)]

use crate::{preprocessor::PreprocessorError, compiler::Compiler, parser::ParseError, codegen::CodegenError};
pub enum CompilerError
{
    BadFilename(String),
    PreprocessorError(PreprocessorError),
    ParseError(ParseError),
    CodegenError(CodegenError)
}

impl std::fmt::Display for CompilerError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            CompilerError::BadFilename(name) => write!(f, "Unable to open file {}", name),
            CompilerError::PreprocessorError(error) => write!(f, "Preprocessor error: {}", error),
            CompilerError::ParseError(error) => write!(f, "Parse error: {}", error),
            CompilerError::CodegenError(error) => write!(f, "Codegen error: {}", error)
        }
    }
}

impl std::convert::From<PreprocessorError> for CompilerError
{
    fn from(error: PreprocessorError) -> Self
    {
        CompilerError::PreprocessorError(error)
    }
}

impl std::convert::From<ParseError> for CompilerError
{
    fn from(error: ParseError) -> Self
    {
        CompilerError::ParseError(error)
    }
}

impl std::convert::From<CodegenError> for CompilerError
{
    fn from(error: CodegenError) -> Self
    {
        CompilerError::CodegenError(error)
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
            CompilerError::ParseError(error) =>
            {
                if let Ok(file) = compiler.get_file_manager(&error.location.filename)
                {
                    file.display_arrow(&error.location, error.arrow_length);
                }
            },
            CompilerError::CodegenError(error) =>
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