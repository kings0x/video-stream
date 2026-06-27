use crate::probe::VideoMeta;

#[derive(Debug, Clone)]
pub struct Rendition {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub bitrate_kbps: u32,
    pub crf: u8,
}

//slice used instead of vec, because tier table never changes at runtime
pub const TIER_TABLE: &[Rendition] = &[
    Rendition {
        name: "144p",
        width: 256,
        height: 144,
        bitrate_kbps: 200,
        crf: 28,
    },
    Rendition {
        name: "240p",
        width: 426,
        height: 240,
        bitrate_kbps: 400,
        crf: 27,
    },
    Rendition {
        name: "360p",
        width: 640,
        height: 360,
        bitrate_kbps: 800,
        crf: 25,
    },
    Rendition {
        name: "480p",
        width: 854,
        height: 480,
        bitrate_kbps: 1400,
        crf: 23,
    },
    Rendition {
        name: "720p",
        width: 1280,
        height: 720,
        bitrate_kbps: 2800,
        crf: 22,
    },
    Rendition {
        name: "1080p",
        width: 1920,
        height: 1080,
        bitrate_kbps: 5000,
        crf: 21,
    },
];

pub fn build_ladder(source_height: u32) -> Vec<Rendition> {
    let mut ladder: Vec<Rendition> = TIER_TABLE
        .iter()
        .filter(|tier| tier.height <= source_height)
        .cloned()
        .collect();
    if ladder.is_empty() {
        if let Some(lowest) = TIER_TABLE.first() {
            ladder.push(lowest.clone());
        }
    }
    ladder
}

pub fn build_ladder_from_meta(meta: &VideoMeta) -> Vec<Rendition> {
    build_ladder(meta.height as u32)
}
