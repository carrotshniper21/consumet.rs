use crate::models::IVideo;
use serde::Deserialize;

/// Contains the Decrypted Sources
#[derive(Debug, Deserialize)]
pub struct StreamTape {
    pub sources: Vec<IVideo>,
}

impl StreamTape {
    const SERVER_NAME: &'static str = "StreamTape";

    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<StreamTape> {
        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        Ok(StreamTape {
            sources: self.sources.clone(),
        })
    }
}
