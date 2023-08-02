use crate::models::{ExtractConfig, IVideo, VideoExtractor};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bilibili {
    pub sources: Vec<IVideo>,
}

impl VideoExtractor for Bilibili {
    type VideoSource = Bilibili;

    // NOTE: Only needs episode_id param
    async fn extract(
        &mut self,
        _episode_id: String,
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
