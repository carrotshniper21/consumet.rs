use crate::models::{ISubtitle, IVideo, Intro};
use crate::utils::util_funcs::USER_AGENT;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FileMoon {
    pub sources: Vec<IVideo>,
}

const HOST: &str = "https://filemoon.sx";

impl FileMoon {
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
