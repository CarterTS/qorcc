use clap::Parser;

/// Argument based compiler options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CompilerSettings
{
    #[clap(short='T', long="tokens")]
    pub dump_tokens: bool,
    pub filenames: Vec<String>,
}

impl std::default::Default for CompilerSettings
{
    fn default() -> Self
    {
        CompilerSettings
        {
            filenames: vec![],
            dump_tokens: false
        }
    }
}