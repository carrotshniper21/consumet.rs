use super::base_parser::BaseParser;
use super::types::{IAnimeInfo, IEpisodeServer, ISource};

use async_trait::async_trait;

#[async_trait]
pub trait AnimeParser: BaseParser {
    const IS_DUB_AVAILABLE_SEPARATELY: bool;
    async fn fetch_anime_info(
        &self,
        anime_id: String,
    ) -> Result<IAnimeInfo, Box<dyn std::error::Error>>;

    async fn fetch_episode_sources(
        &self,
        episode_id: String,
    ) -> Result<ISource, Box<dyn std::error::Error>>;

    async fn fetch_episode_servers(
        &self,
        episode_id: String,
    ) -> Result<Vec<IEpisodeServer>, Box<dyn std::error::Error>>;
}
