use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

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
            trace!("Loading File {}", filename);

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
        self.get_file_manager(filename)?;

        // Pass the file on to the preprocessor
        let mut preprocessor_context = preprocessor::PreprocessorContext::with_compiler(self);
        let tokens = preprocessor_context.preprocess(filename)?;

        // Display the preprocessed tokens if requested
        if self.settings.dump_tokens
        {
            println!("Preprocessed Tokens:");
            for token in &tokens
            {
                println!("   {}", token);
            }
        }

        // Parse the token stream
        let mut parser_context = parser::Parser::from_stream(tokens.iter());
        let tree = parser_context.parse()?;

        // Display the parse tree if requested
        if self.settings.dump_parse_tree
        {
            tree.display();
        }

        // Convert to intermediate representation
        let ir = codegen::parse_tree_to_ir(tree)?;

        // Display the intermediate representation if requested
        if self.settings.dump_intermediate_representation
        { 
            ir.display();
        }

        // Produce assembly from the intermediate representation
        let asm_generator = codegen::AssemblyCodeGenerator::from_ir(ir);
        let asm = asm_generator.codegen()?;

        // Display the assembly output if requested
        if self.settings.dump_assembly
        {
            print!("{}", asm);
        }
        
        // Finally, we get the name of the output file, and write the generated assembly output if the no-out-file flag is not set
        if !self.settings.supress_output
        {
            let filename = Path::new(&filename).file_name().unwrap().to_str().unwrap().split(".").nth(0).unwrap();
            let mut f = std::fs::File::create(format!("{}.s", filename)).unwrap();

            f.write_all(asm.as_bytes()).unwrap();
        }

        Ok(())
    }
}