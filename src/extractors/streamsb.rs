use crate::models::IVideo;
use crate::utils::util_funcs::USER_AGENT;

#[derive(Debug)]
pub struct StreamSB {
    sources: Vec<IVideo>,
}

const HOST: &str = "https://streamsss.net/sources50";
const HOST2: &str = "https://watchsb.com/sources50";

impl StreamSB {
    const SERVER_NAME: &'static str = "StreamSB";

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
