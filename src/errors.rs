#![allow(dead_code)]
pub enum CompilerError
{
    BadFilename(String)
}

impl std::fmt::Display for CompilerError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            CompilerError::BadFilename(name) => write!(f, "Unable to open file {}", name)
        }
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;