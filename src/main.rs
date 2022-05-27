#[macro_use]
extern crate log;

mod codegen;
mod compiler;
mod errors;
mod parser;
mod preprocessor;
mod tokenizer;
mod settings;

fn main()
{
    use env_logger::Env;

    let env = Env::default().filter_or("QOR_CC_LOG", "trace");
    env_logger::init_from_env(env);

    use clap::Parser;
    let compiler_settings = settings::CompilerSettings::parse();

    let mut compiler_instance = compiler::Compiler::with_settings(&compiler_settings);

    for filename in &compiler_settings.filenames
    {
        match compiler_instance.compile(filename)
        {
            Ok(_) => {},
            Err(e) => 
            {
                eprintln!("Compiler Error:\n{}", e);
            }
        }
    }
}
