use crate::models::{ExtractConfig, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmashyStreamSources {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmashyStreamSubtitles {
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmashyStream {
    pub sources: Vec<SmashyStreamSources>,
    pub subtitles: Vec<SmashyStreamSubtitles>,
}

const _HOST: &str = "https://embed.smashystream.com";

impl VideoExtractor for SmashyStream {
    type VideoSource = SmashyStream;

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

        self.sources.push(SmashyStreamSources {
            url: String::new(),
            quality: String::new(),
            is_m3u8: false,
        });

        self.subtitles.push(SmashyStreamSubtitles {
            url: String::new(),
            lang: String::new(),
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
