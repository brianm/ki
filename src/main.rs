/*
   Copyright 2023 Brian McCallister

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use kcci::ingest;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

/// A simple CLI for the kcci library
///
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ingest,
    Woof,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env("KCCI_LOG");
    let dirs = ProjectDirs::from("org", "skife", "kcci")
        // TODO (brianm) maybe just use a temp dir?
        .unwrap_or_else(|| panic!("unable to find cache and data dirs, please set XDG_*"));
    let cache_dir = dirs.cache_dir();
    let data_dir = dirs.data_dir();
    log::info!("cache:{:?}\tdata:{:?}", cache_dir, data_dir);

    let args = Args::parse();
    match args.command {
        Commands::Ingest => {
            let mut reader = std::io::stdin().lock();
            let out = ingest::parse_paste(&mut reader).unwrap();
            for c in out {
                println!("{}\t{}", c.title(), c.authors().join(", "));
            }
        }
        Commands::Woof => {
            let model =
                SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
                    .create_model()
                    .unwrap();

            let sentences = ["this is an example sentence", "each sentence is converted"];

            let _output = model.encode(&sentences);
        }
    }
    Ok(())
}
