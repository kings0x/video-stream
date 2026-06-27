use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use thiserror::Error;
use tokio::task::spawn_blocking;

use crate::ladder::Rendition;

#[derive(Debug, Error)]
pub enum TranscodeError {
    #[error("ffmpeg not found or failed to spawn: {0}")]
    SpawnFailed(String),

    #[error("ffmpge exited with error for rendition {rendition} : {stderr}")]
    EncodedFailed { rendition: String, stderr: String },

    #[error("failed to create output directory: {0}")]
    OutputDirFailed(String),
}

pub fn prepare_output_dir(job_id: &str, rendition_name: &str) -> Result<PathBuf, TranscodeError> {
    let dir = PathBuf::from("output").join(job_id).join(rendition_name);
    create_dir_all(&dir).map_err(|e| TranscodeError::OutputDirFailed(e.to_string()))?;
    Ok(dir)
}

pub fn build_ffmpeg_args(input_path: &Path, rendition: &Rendition, out_dir: &Path) -> Vec<String> {
    let playlist_path = out_dir.join("playlist.m3u8");
    let segment_pattern = out_dir.join("segement_%03d.ts");

    vec![
        "-i".to_string(),
        input_path.to_string_lossy().to_string(),
        "-vf".to_string(),
        format!("scale=-2:{}", rendition.height),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        rendition.crf.to_string(),
        "-maxrate".to_string(),
        format!("{}k", rendition.bitrate_kbps),
        "-bufsize".to_string(),
        format!("{}k", rendition.bitrate_kbps * 2),
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        "-hls_time".to_string(),
        "6".to_string(),
        "-hls_playlist_type".to_string(),
        "vod".to_string(),
        "-hls_segment_filename".to_string(),
        segment_pattern.to_string_lossy().to_string(),
        playlist_path.to_string_lossy().to_string(),
    ]
}

pub fn transcode_rendition(
    input_path: &Path,
    rendition: &Rendition,
    job_id: &str,
) -> Result<PathBuf, TranscodeError> {
    let out_dir = prepare_output_dir(job_id, rendition.name)?;

    let args = build_ffmpeg_args(input_path, rendition, &out_dir);

    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|e| TranscodeError::SpawnFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        return Err(TranscodeError::EncodedFailed {
            rendition: rendition.name.to_string(),
            stderr,
        });
    }

    Ok(out_dir)
}

pub async fn transcode_all(
    input_path: PathBuf,
    job_id: String,
    renditions: Vec<Rendition>,
) -> Vec<Result<PathBuf, TranscodeError>> {
    let mut handles = Vec::new();

    let mut results = Vec::new();

    let input_path = Arc::new(input_path);

    let job_id = Arc::new(job_id);

    for rendition in renditions {
        let input_path = input_path.clone();
        let job_id = job_id.clone();
        let handle = spawn_blocking(move || {
            let val = transcode_rendition(&input_path, &rendition, &job_id);

            val
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(val) => {
                results.push(val);
            }

            Err(e) => {
                results.push(Err(TranscodeError::SpawnFailed(e.to_string())));
            }
        }
    }

    results
}
