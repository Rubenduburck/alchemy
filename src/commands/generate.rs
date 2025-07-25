use crate::client::Client;
use crate::commands::SubCommand;
use crate::types::CliResult;
use clap::Args;

#[derive(Args)]
pub struct GenerateCommand {
    /// Encoding type
    #[arg(short, long)]
    pub encoding: String,
    /// Number of bytes
    #[arg(short, long)]
    pub bytes: u64,
}

impl SubCommand for GenerateCommand {
    fn run(&self, _list_mode: bool) -> CliResult {
        let client = Client::new();
        client.generate(&self.encoding, self.bytes as usize).into()
    }
}
