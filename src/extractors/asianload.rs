use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AsianLoadSource {
    pub url: String,
    pub is_m3u8: bool,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AsianLoadSubtitle {
    pub url: String,
    pub lang: String,
}

/// Contains both the Decrypted Sources and Subtitles
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AsianLoad {
    pub sources: Vec<AsianLoadSource>,
    pub subtitles: Vec<AsianLoadSubtitle>,
}

impl VideoExtractor for AsianLoad {
    type VideoSource = AsianLoad;

    // NOTE: Only needs video_url param
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

        self.sources.push(AsianLoadSource {
            url: String::new(),
            is_m3u8: false,
        });

        self.subtitles.push(AsianLoadSubtitle {
            url: String::new(),
            lang: String::new(),
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
