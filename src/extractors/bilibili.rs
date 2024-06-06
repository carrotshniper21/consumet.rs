use crate::models::{ExtractConfig, VideoExtractor};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BilibiliSources {
    pub url: String,
    pub is_m3u8: bool,
    pub is_dash: bool,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bilibili {
    pub sources: Vec<BilibiliSources>,
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

        self.sources.push(BilibiliSources {
            url: String::new(),
            is_m3u8: false,
            is_dash: false,
        });

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
