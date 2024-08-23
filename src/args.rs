use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpellChecker {
    command_type: CommandType,
}

#[derive(Subcommand)]
pub enum CommandType {}
