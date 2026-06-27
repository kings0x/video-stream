mod ladder;
mod probe;
mod transcode;
use std::env;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::ladder::build_ladder_from_meta;
use crate::probe::ProbeError;
use crate::transcode::{TranscodeError, transcode_all};

#[tokio::main]
async fn main() -> Result<(), ProbeError> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("path to video not specified");
    }

    let path = Path::new(&args[1]);

    match probe::probe(path) {
        Ok(video_meta) => {
            let renditions = build_ladder_from_meta(&video_meta);
            let job_id = Uuid::new_v4().to_string();

            let paths = transcode_all(path.to_path_buf(), job_id, renditions).await;

            handle_failures(&paths);
        }
        Err(e) => {
            eprintln!("Error Occured: {}", e)
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
