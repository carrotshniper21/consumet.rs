use crate::models::{ISubtitle, IVideo};

#[derive(Debug)]
pub struct MixDrop {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

impl MixDrop {
    const SERVER_NAME: &'static str = "MixDrop";

    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<MixDrop> {
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

        Ok(MixDrop {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone()
        })
    }
}
