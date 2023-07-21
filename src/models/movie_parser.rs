use crate::models::{BaseParser, ISearch, TvType, IMovieInfo};

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

    async fn fetch_media_info(
        &self,
        media_id: String
    ) -> anyhow::Result<IMovieInfo>;

    // async fn fetch_episode_servers(
    //     &self,
    //     episode_id: String,
    // ) -> anyhow::Result<Vec<IEpisodeServer>

    // async fn fetch_episode_sources(
    //     &self,
    //     episode_id: String,
    // ) -> anyhow::Result<ISource>;
}
