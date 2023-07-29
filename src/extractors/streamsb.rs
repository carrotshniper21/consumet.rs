use crate::models::IVideo;
use crate::utils::util_funcs::USER_AGENT;
use serde::Deserialize;

/// Contains the Decrypted Sources
#[derive(Debug, Deserialize)]
pub struct StreamSB {
    pub sources: Vec<IVideo>,
}

const _HOST: &str = "https://streamsss.net/sources50";
const _HOST2: &str = "https://watchsb.com/sources50";

impl StreamSB {
    pub async fn extract(&mut self, _video_url: String) -> anyhow::Result<StreamSB> {
        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        Ok(StreamSB {
            sources: self.sources.clone(),
        })
    }
}
