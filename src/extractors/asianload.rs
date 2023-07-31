use crate::models::{ISubtitle, IVideo};
use serde::Deserialize;

/// Contains both the Decrypted Sources and Subtitles
#[derive(Debug, Deserialize)]
pub struct AsianLoad {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

impl AsianLoad {
    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<Self> {
        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        self.subtitles.push(ISubtitle {
            id: None,
            url: None,
            lang: None,
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
