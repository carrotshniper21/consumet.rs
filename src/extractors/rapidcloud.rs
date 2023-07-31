use crate::models::{ISubtitle, IVideo, Intro, ProxyConfig};
use crate::utils::util_funcs::UtilFuncs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RapidCloud {
    sources: Vec<IVideo>,
    subtitles: Vec<ISubtitle>,
}

const HOST: &str = "https://rapid-cloud.co";

impl RapidCloud {
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
