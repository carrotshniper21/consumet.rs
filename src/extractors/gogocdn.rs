use crate::models::{IVideo, ProxyConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GogoCDN {
    pub sources: Vec<IVideo>,
}

impl GogoCDN {
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
