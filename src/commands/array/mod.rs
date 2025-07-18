use crate::commands::SubCommand;
use crate::types::CliResult;
use clap::{Args, Subcommand};

pub mod chunk;
pub mod flatten;
pub mod reverse;
pub mod rotate;
pub mod truncate;

use chunk::ChunkCommand;
use flatten::FlattenCommand;
use reverse::ReverseCommand;
use rotate::RotateCommand;
use truncate::TruncateCommand;

#[derive(Args)]
pub struct ArrayCommand {
    #[command(subcommand)]
    pub command: ArrayCommands,
}

#[derive(Subcommand)]
pub enum ArrayCommands {
    /// Flatten a nested array
    Flatten(FlattenCommand),
    /// Chunk an array into specified number of chunks
    Chunk(ChunkCommand),
    /// Reverse an array at specified depth
    Reverse(ReverseCommand),
    /// Rotate an array left or right
    Rotate(RotateCommand),
    /// Truncate an array from little end (default) or big end
    Truncate(TruncateCommand),
}

impl SubCommand for ArrayCommand {
    fn run(&self, list_mode: bool) -> CliResult {
        match &self.command {
            ArrayCommands::Flatten(cmd) => cmd.run(list_mode),
            ArrayCommands::Chunk(cmd) => cmd.run(list_mode),
            ArrayCommands::Reverse(cmd) => cmd.run(list_mode),
            ArrayCommands::Rotate(cmd) => cmd.run(list_mode),
            ArrayCommands::Truncate(cmd) => cmd.run(list_mode),
        }
    }
}
