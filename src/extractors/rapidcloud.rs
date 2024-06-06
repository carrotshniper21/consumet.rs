use crate::models::{ExtractConfig, VideoExtractor};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RapidCloudSources {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RapidCloudSubtitles {
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RapidCloud {
    pub sources: Vec<RapidCloudSources>,
    pub subtitles: Vec<RapidCloudSubtitles>,
}

const _HOST: &str = "https://rapid-cloud.co";

impl VideoExtractor for RapidCloud {
    type VideoSource = RapidCloud;

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

        self.sources.push(RapidCloudSources {
            url: String::new(),
            quality: String::new(),
            is_m3u8: false,
        });

        self.subtitles.push(RapidCloudSubtitles {
            url: String::new(),
            lang: String::new(),
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
