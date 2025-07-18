#![allow(clippy::uninlined_format_args)]
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use crate::types::CliResult;
use clap::Args;
use std::io::{self, Read};

#[derive(Args)]
pub struct TruncateCommand {
    /// Length to truncate to
    #[arg(long)]
    pub length: usize,
    /// Truncate from the big end (right side) instead of little end (left side)
    #[arg(short, long)]
    pub big_end: bool,
    /// Input data to truncate
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for TruncateCommand {
    fn run(&self, _list_mode: bool) -> CliResult {
        let input = if self.input.is_empty() {
            // Read from stdin if no arguments provided
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer.trim().to_string(),
                Err(e) => {
                    return Error::Generic(format!("Failed to read from stdin: {}", e)).into()
                }
            }
        } else {
            self.input.join(" ")
        };

        if input.is_empty() {
            return Error::MissingArgs("No input provided".to_string()).into();
        }
        let client = Client::new();
        client.truncate_array(&input, self.length, self.big_end).into()
    }
}