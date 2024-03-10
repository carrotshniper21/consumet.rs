use crate::models::{ExtractConfig, IVideo, VideoExtractor};

use serde::{Deserialize, Serialize};

/// Contains the Decrypted Sources
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamSB {
    pub sources: Vec<IVideo>,
}

const _HOST: &str = "https://streamsss.net/sources50";
const _HOST2: &str = "https://watchsb.com/sources50";

impl VideoExtractor for StreamSB {
    type VideoSource = StreamSB;

    // NOTE: Only needs video_url & is_alternative param
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
