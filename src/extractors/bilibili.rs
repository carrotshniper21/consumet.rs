use crate::models::{ISource, IVideo};
use crate::utils::util_funcs::UtilFuncs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Bilibili {
    pub sources: Vec<IVideo>,
}

impl Bilibili {
    pub async fn extract(&mut self, _episode_id: String) -> anyhow::Result<Self> {
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
