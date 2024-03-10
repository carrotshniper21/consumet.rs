use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MixDropSource {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MixDropSubtitle {
    pub url: String,
    pub lang: String,
}

/// Contains both the Decrypted Sources and Subtitles
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MixDrop {
    pub sources: Vec<MixDropSource>,
    pub subtitles: Vec<MixDropSubtitle>,
}

impl VideoExtractor for MixDrop {
    type VideoSource = MixDrop;

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

        self.sources.push(MixDropSource {
            url: String::new(),
            quality: String::new(),
            is_m3u8: false,
        });

        self.subtitles.push(MixDropSubtitle {
            url: String::new(),
            lang: String::new(),
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
