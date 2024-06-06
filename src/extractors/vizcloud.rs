use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VizCloudSources {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VizCloud {
    pub sources: Vec<VizCloudSources>,
}

const _HOST: &str = "https://vidstream.pro";

impl VideoExtractor for VizCloud {
    type VideoSource = VizCloud;

    // NOTE: Only needs video_url & viz_cloud_helper & api_key param
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

        self.sources.push(VizCloudSources {
            url: String::new(),
            quality: String::new(),
            is_m3u8: false,
        });

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
