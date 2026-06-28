use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "streamer")]
#[command(about = "adaptive bitrate streaming server")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Transcode a video file into HLS renditions
    Transcode {
        /// Path to the source file
        path: PathBuf,
    },

    /// Start the HTTP server to server transcoded streams
    Server {
        /// Port to listen on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}
