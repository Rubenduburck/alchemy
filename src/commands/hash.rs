use crate::client::Client;
use crate::commands::SubCommand;
use crate::encode::encoding::Encoding;
use crate::error::Error;
use crate::types::{CliResult, HashResponse, HashResult, Response};
use clap::Args;
use std::collections::HashMap;

#[derive(Args)]
pub struct HashCommand {
    /// Hash algorithm(s) - defaults to keccak256
    #[arg(short, long, value_delimiter = ',', default_value = "keccak256")]
    pub algo: Vec<String>,
    /// Input encoding(s) - if not specified, will auto-classify
    #[arg(short, long, value_delimiter = ',')]
    pub input_encoding: Option<Vec<String>>,
    /// Input data to hash
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for HashCommand {
    fn run(&self, list_mode: bool) -> CliResult {
        if self.input.is_empty() {
            return Error::MissingArgs("input".to_string()).into();
        }
        let input = self.input.join(" ");
        let client = Client::new();

        // Determine input encodings
        let encodings = match &self.input_encoding {
            Some(encodings) => encodings.clone(),
            None => {
                // Auto-classify if no input encoding provided
                let mut classifications = client.classify(&input);
                classifications.retain(|c| !c.is_empty());
                classifications.sort();
                classifications
                    .iter()
                    .map(|c| c.encoding().to_string())
                    .collect()
            }
        };

        if !list_mode && self.algo.len() == 1 {
            // Single algorithm, non-list mode: return just the hash using best encoding
            let algo_name = &self.algo[0];
            let encoding = &encodings[0]; // Use the best encoding (first in sorted list)
            client
                .hash(algo_name, &input, Encoding::from(encoding))
                .map(|hash| hash.to_string())
                .into()
        } else {
            // Multiple algorithms/encodings or list mode: return full structure
            let results = encodings
                .iter()
                .flat_map(|encoding| {
                    let values = self
                        .algo
                        .iter()
                        .flat_map(|algo| {
                            client
                                .hash(algo, &input, Encoding::from(encoding))
                                .ok()
                                .map(|output| {
                                    (
                                        algo.clone(),
                                        HashResult {
                                            output: output.to_string(),
                                        },
                                    )
                                })
                        })
                        .collect::<HashMap<String, HashResult>>();
                    if values.is_empty() {
                        None
                    } else {
                        Some((encoding.clone(), values))
                    }
                })
                .collect::<HashMap<String, HashMap<String, HashResult>>>();
            Response::Hash(HashResponse::Multiple(results)).into()
        }
    }
}
