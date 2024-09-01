use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SpellChecker {
    #[clap(subcommand)]
    pub command_type: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Checks the file for spelling errors and mistakes
    Check(CheckerArguments),
}

#[derive(Debug, Args)]
pub struct CheckerArguments {
    /// some function which is supposed to do something
    pub some_func: bool,
}
