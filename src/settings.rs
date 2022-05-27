use clap::Parser;

/// Argument based compiler options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CompilerSettings
{
    pub filenames: Vec<String>,
}

impl std::default::Default for CompilerSettings
{
    fn default() -> Self
    {
        CompilerSettings
        {
            filenames: vec![],
        }
    }
}