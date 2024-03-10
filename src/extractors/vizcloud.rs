use crate::models::{ExtractConfig, IVideo, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VizCloud {
    sources: Vec<IVideo>,
}

const _HOST: &str = "https://vidstream.pro";

impl VideoExtractor for VizCloud {
    type VideoSource = VizCloud;

    // NOTE: Only needs video_url & viz_cloud_helper & api_key param
    async fn extract(
        &mut self,
        _video_url: String,
        _args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
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
