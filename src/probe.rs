use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProbeError {
    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("ffprobe failed: {0}")]
    FFprobeFailed(String),

    #[error("no video stream found in file")]
    NoVideoStream,

    #[error("failed to parse ffprobe output: {0}")]
    ParseError(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FFprobeOutput {
    streams: Vec<FFprobeStreams>,
    format: FFprobeFormat,
}

#[derive(Debug, Serialize, Deserialize)]
struct FFprobeStreams {
    codec_type: Option<String>,
    width: Option<u64>,
    height: Option<u64>,
    r_frame_rate: Option<String>,
    bit_rate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FFprobeFormat {
    duration: Option<String>,
    bit_rate: Option<String>,
}
pub struct VideoMeta {
    pub width: u64,
    pub height: u64,
    pub framerate: f64,
    pub bitrate_kbps: u64,
    pub duration_secs: f64,
}

impl VideoMeta {
    pub fn summary(&self) -> String {
        format!(
            "{} x {} @ {:.2}fps | {:.0}kbps | {:.1}s",
            self.width, self.height, self.framerate, self.bitrate_kbps, self.duration_secs,
        )
    }
}

pub fn probe(path: &Path) -> Result<VideoMeta, ProbeError> {
    if !path.exists() {
        return Err(ProbeError::FileNotFound(path.to_string_lossy().to_string()));
    }

    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_streams",
            "-show_format",
            path.to_str().unwrap_or_default(),
        ])
        .output()
        .map_err(|e| ProbeError::FFprobeFailed(e.to_string()))?;

    if !output.status.success() {
        let stdeerr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ProbeError::FFprobeFailed(stdeerr));
    }

    let ffprobe_output: FFprobeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|e| ProbeError::ParseError(e.to_string()))?;

    let video_stream = ffprobe_output
        .streams
        .iter()
        .find(|val| val.codec_type.as_deref() == Some("video"))
        .ok_or(ProbeError::NoVideoStream)?;

    let width = video_stream
        .width
        .ok_or(ProbeError::ParseError("missing width".to_string()))?;

    let height = video_stream
        .height
        .ok_or(ProbeError::ParseError("missing height".to_string()))?;

    let framerate = {
        let framerate: Vec<&str> = video_stream
            .r_frame_rate
            .as_deref()
            .unwrap_or("30/1")
            .split("/")
            .collect();

        let num = framerate[0].parse::<f64>().unwrap_or(30.0);
        let den = framerate[1].parse::<f64>().unwrap_or(1.0);

        if den == 0.0 { 30.0 } else { num / den }
    };
    let bitrate_kbps = video_stream
        .bit_rate
        .as_deref()
        .or(ffprobe_output.format.bit_rate.as_deref())
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
        / 1000;

    let duration_secs = ffprobe_output
        .format
        .duration
        .as_deref()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);

    let vid_meta = VideoMeta {
        width,
        height,
        framerate,
        bitrate_kbps,
        duration_secs,
    };

    Ok(vid_meta)
}
