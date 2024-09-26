// Local Imports
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CliArgument {
    #[clap(subcommand)]
    pub command_type: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Checks the file for spelling errors and mistakes
    Check(CheckerArguments),
    /// Checks if a word exists or not and give suggestions on misspelled words
    Is(IsArgument),
}

#[derive(Debug, Args)]
pub struct CheckerArguments {
    /// Some file ot be spell checked
    pub word_to_be: String,
}

#[derive(Debug, Args)]
pub struct IsArgument {
    /// To check if the word is real or not.
    pub word_to_check: String,
}
