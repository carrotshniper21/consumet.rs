use crate::models::{ISource, IVideo};

pub struct StreamLare {
    sources: Vec<IVideo>,
}

const HOST: &str = "https://streamlare.com";

impl StreamLare {
    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<Self> {
        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
