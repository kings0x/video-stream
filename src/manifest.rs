use std::{
    fs::{self, create_dir_all},
    io,
    path::{Path, PathBuf},
};

use crate::ladder::Rendition;

fn build_master_playlist(renditions: &[Rendition]) -> String {
    let mut playlist = String::from("#EXTM3U\n#EXT-X-VERSION:3\n");

    for r in renditions {
        let bandwidth_bps = r.bitrate_kbps as u32 * 1000;

        playlist.push_str(&format!(
            "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{}\n{}/playlist.m3u8\n",
            bandwidth_bps, r.width, r.height, r.name
        ));
    }

    playlist
}

pub fn write_master_playlist(job_id: &str, renditions: &[Rendition]) -> io::Result<PathBuf> {
    let content = build_master_playlist(renditions);
    let path = Path::new("output").join(job_id).join("master.m3u8");

    fs::write(&path, content)?;

    Ok(path)
}
