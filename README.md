# VIDEO-STREAM

> An adaptive bitrate (ABR) streaming server written in Rust. It probes a source video, generates multiple resolution renditions (144p–1080p), packages them as HLS, and serves them with quality switching driven by either the user or network conditions — the same approach used by YouTube, Netflix, and
Twitch under the hood.


## Demo & Screenshots

## About

Adaptive bitrate streaming is the technique behind every major video
platform: instead of serving one fixed-quality file, the source is
encoded into several resolution/bitrate tiers, and the player switches
between them in real time based on the viewer's available bandwidth.

This project implements that pipeline from scratch in Rust:

- Inspecting source video metadata with `ffprobe`
- Building a resolution ladder capped by the source's native resolution
- Transcoding to HLS segments with `ffmpeg`
- Serving manifests and segments over HTTP with Axum
- Simulating network conditions to demonstrate quality switching

It's built as a learning project and portfolio piece, prioritizing a
correct, well-understood implementation over comprehensive features.


## Features

- [x] Video metadata extraction (resolution, framerate, bitrate, duration) via `ffprobe`
- [x] Resolution ladder generation, capped to source resolution
- [x] HLS transcoding pipeline (multiple renditions via `ffmpeg`)
- [ ] HLS manifest generation (master + per-rendition playlists)
- [ ] HTTP server for manifests and segments
- [ ] Network-aware adaptive bitrate switching

## TechStack

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, 1.75+)
- [FFmpeg](https://ffmpeg.org/download.html) — provides both `ffmpeg` and `ffprobe`, must be available on your `PATH`
  - macOS: `brew install ffmpeg`
  - Windows: `choco install ffmpeg` or `winget install ffmpeg`
  - Debian/Ubuntu: `sudo apt install ffmpeg`


### Installation

### Usage