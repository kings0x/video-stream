mod cli;
mod ladder;
mod manifest;
mod probe;
mod server;
mod transcode;
use crate::server::Server;
use clap::Parser;
use std::path::PathBuf;
use uuid::Uuid;

use crate::cli::{Cli, Commands};
use crate::ladder::build_ladder_from_meta;
use crate::manifest::write_master_playlist;
use crate::probe::ProbeError;
use crate::transcode::{TranscodeError, transcode_all};

#[tokio::main]
async fn main() -> Result<(), ProbeError> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Transcode { path } => match probe::probe(&path) {
            Ok(video_meta) => {
                video_meta.summary();
                let renditions = build_ladder_from_meta(&video_meta);

                let job_id = Uuid::new_v4().to_string();

                let paths =
                    transcode_all(path.to_path_buf(), job_id.clone(), renditions.clone()).await;

                handle_failures(&paths);

                write_master_playlist(&job_id, &renditions)
                    .map_err(|e| ProbeError::ParseError(e.to_string()))?;
            }
            Err(e) => {
                eprintln!("Error Occured: {}", e)
            }
        },

        Commands::Server { port } => {
            let addr = format!("127.0.0.1:{}", port);

            let server = Server::new(&addr)
                .await
                .map_err(|e| ProbeError::ParseError(e.to_string()))?;

            println!("server listening on http://localhost:{}", port);
            server
                .run()
                .await
                .map_err(|e| ProbeError::ParseError(e.to_string()))?;
        }
    }

    Ok(())
}

pub fn handle_failures(results: &[Result<PathBuf, TranscodeError>]) {
    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for result in results {
        match result {
            Ok(val) => successes.push(val),
            Err(e) => {
                failures.push(e);
            }
        }
    }

    println!(
        "total successful renditions are {} total failed renditions are {}",
        successes.len(),
        failures.len()
    );

    for failure in failures {
        eprintln!("failure -> {}", failure)
    }
}
