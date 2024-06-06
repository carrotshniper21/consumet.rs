use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MegaCloudSources {
    pub url: String,
    pub r#type: String,
    pub is_m3u8: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MegaCloudSubtitles {
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MegaCloud {
    pub sources: Vec<MegaCloudSources>,
    pub subtitles: Vec<MegaCloudSubtitles>,
}

impl VideoExtractor for MegaCloud {
    type VideoSource = MegaCloud;

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

        self.sources.push(MegaCloudSources {
            url: String::new(),
            r#type: String::new(),
            is_m3u8: false,
        });

        self.subtitles.push(MegaCloudSubtitles {
            url: String::new(),
            lang: String::new(),
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
