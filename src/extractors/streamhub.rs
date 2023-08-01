use crate::models::{ExtractConfig, ISubtitle, IVideo, VideoExtractor};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct StreamHub {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

impl VideoExtractor for StreamHub {
    type VideoSource = StreamHub;

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

        self.subtitles.push(ISubtitle {
            id: None,
            url: None,
            lang: None,
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
