use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamTapeSources {
    pub url: String,
    pub is_m3u8: bool,
}

/// Contains the Decrypted Sources
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamTape {
    pub sources: Vec<StreamTapeSources>,
}

impl VideoExtractor for StreamTape {
    type VideoSource = StreamTape;

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

        self.sources.push(StreamTapeSources {
            url: String::new(),
            is_m3u8: false,
        });

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
