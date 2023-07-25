use crate::models::{BaseParser, ISearch, TvType, StreamingServers};

pub enum IInfoType {
    IMovieInfo,
    IAnimeInfo,
}

/// A trait providing movie parsing methods to implment on
pub trait MovieParser: BaseParser {
    type SearchResult;
    type MediaInfo;
    type ServerResult;
    type SourceResult;

    /// The supported types of the provider (e.g. `&[TvType::TvSeries, TvType::Movie]`)
    fn supported_types(&self) -> &[TvType];

    /// Returns a search result from the provided query and page number
    /// ```
    /// use consumet_api_rs::models::MovieParser;
    /// use consumet_api_rs::providers::movies;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     println!(
    ///         "{:#?}",
    ///          movies::FlixHQ.search("puss".to_owned(), Some(1)).await?
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<Self::SearchResult>>;

    /// Returns more info for the provided media id
    /// ```
    /// use consumet_api_rs::models::MovieParser;
    /// use consumet_api_rs::providers::movies;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     println!(
    ///         "{:#?}",
    ///         movies::FlixHQ
    ///             .fetch_media_info("tv/watch-phineas-and-ferb-39100".to_owned())
    ///             .await?
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<Self::MediaInfo>;

    /// Returns server info for provided episode id and media_id
    /// ```
    /// use consumet_api_rs::models::MovieParser;
    /// use consumet_api_rs::providers::movies;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     println!(
    ///         "{:#?}",
    ///         movies::FlixHQ
    ///             .fetch_episode_servers(
    ///                 "98488".to_owned(),
    ///                 "movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488"
    ///                .    to_owned()
    ///             )
    ///             .await?
    ///     );

    ///     Ok(())
    /// }
    /// ```
    async fn fetch_episode_servers(
        &self,
        episode_id: String,
        media_id: String,
    ) -> anyhow::Result<Vec<Self::ServerResult>>;

    async fn fetch_episode_sources(
        &self,
        episode_id: String,
        media_id: String,
        server: Option<StreamingServers>
    ) -> anyhow::Result<Self::SourceResult>;
}
