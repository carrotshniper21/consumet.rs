use crate::models::{ExtractConfig, IVideo, VideoExtractor};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FileMoon {
    pub sources: Vec<IVideo>,
}

const HOST: &str = "https://filemoon.sx";

impl VideoExtractor for FileMoon {
    type VideoSource = FileMoon;

    async fn extract(
        &mut self,
        _video_url: String,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
        let ExtractConfig {
            vis_cloud_helper: _,
            api_key: _,
            is_alternative: _,
            user_agent: _,
        } = args;

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
