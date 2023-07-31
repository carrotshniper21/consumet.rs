use crate::models::{ISubtitle, IVideo, Intro};

pub struct VizCloud {
    sources: Vec<IVideo>,
}

const HOST: &str = "https://vidstream.pro";

impl VizCloud {
    pub async fn extract(&mut self, video_url: String) -> anyhow::Result<Self> {
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
