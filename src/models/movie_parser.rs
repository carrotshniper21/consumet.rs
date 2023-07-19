use super::base_parser::BaseParser;
use super::types::{ISearch, TvType};

use async_trait::async_trait;
pub enum IInfoType {
    IMovieInfo,
    IAnimeInfo,
}

#[async_trait]
pub trait MovieParser: BaseParser {
    type SearchResult;

    /// The supported types of the provider (e.g. `&[TvType::TvSeries, TvType::Movie]`)
    fn supported_types(&self) -> &[TvType];

    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<Self::SearchResult>>;
    // async fn fetch_media_info(
    //     &self,
    //     media_id: String,
    //     media_type: String,
    // ) -> Result<IInfoType, Box<dyn std::error::Error>>;
    // async fn fetch_episode_servers(
    //     &self,
    //     episode_id: String,
    // ) -> Result<Vec<IEpisodeServer>, Box<dyn std::error::Error>>;
    // async fn fetch_episode_sources(
    //     &self,
    //     episode_id: String,
    // ) -> Result<ISource, Box<dyn std::error::Error>>;
}
