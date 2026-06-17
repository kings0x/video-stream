mod probe;
use std::path::Path;

use crate::probe::ProbeError;

fn main() -> Result<(), ProbeError> {
    tracing_subscriber::fmt::init();
    let path = Path::new("./videos/docker_vid_1.mp4");
    match probe::probe(path) {
        Ok(video_meta) => {
            let vid = video_meta.summary();
            println!("{}", vid);
        }
        Err(e) => {
            eprintln!("Error Occured: {}", e)
        }
    }

    Ok(())
}
