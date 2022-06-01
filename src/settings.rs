use clap::Parser;

/// Argument based compiler options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CompilerSettings
{
    #[clap(short='T', long="tokens")]
    pub dump_tokens: bool,
    #[clap(short='P', long="parsetree")]
    pub dump_parse_tree: bool,
    #[clap(short='I', long="intermediate")]
    pub dump_intermediate_representation: bool,
    #[clap(short='A', long="assembly")]
    pub dump_assembly: bool,
    #[clap(short='S', long="no-out")]
    pub supress_output: bool,
    pub filenames: Vec<String>,
}

impl std::default::Default for CompilerSettings
{
    fn default() -> Self
    {
        CompilerSettings
        {
            filenames: vec![],
            dump_tokens: false,
            dump_parse_tree: false,
            dump_intermediate_representation: false,
            dump_assembly: false,
            supress_output: false
        }
    }
}