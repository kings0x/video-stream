mod ladder;
mod probe;
use std::env;
use std::path::Path;

use crate::ladder::build_ladder_from_meta;
use crate::probe::ProbeError;

fn main() -> Result<(), ProbeError> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("path to video not specified");
    }

    let path = Path::new(&args[1]);

    match probe::probe(path) {
        Ok(video_meta) => {
            let renditions = build_ladder_from_meta(&video_meta);
            let vid = video_meta.summary();
            println!("details :-> {}\nrenditions :-> {:#?}", vid, renditions);
        }
        Err(e) => {
            eprintln!("Error Occured: {}", e)
        }
    }

    Ok(())
}
