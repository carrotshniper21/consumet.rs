use crate::models::{IVideo, ISubtitle};

/// Contains both the Decrypted Sources and Subtitles
#[derive(Debug)]
pub struct AsianLoad {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>
}

impl AsianLoad {
    const SERVER_NAME: &'static str = "AsianLoad";

    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<AsianLoad> {
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

        Ok(AsianLoad {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone()
        })
    }
}