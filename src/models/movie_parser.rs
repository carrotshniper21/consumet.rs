use crate::models::{BaseParser, ISearch, StreamingServers, TvType};

/// A trait providing movie parsing methods to implement on
/// ```
/// use consumet_api_rs::models::MovieParser;
/// use consumet_api_rs::providers::movies;
///
/// // <provider_name> is the name of the provider you want to use.
/// let movie_provider = movies::<provider_name>;
/// ```
pub trait MovieParser: BaseParser {
    type MediaInfo;
    type ServerResult;
    type SourceResult;

    /// The supported types of the provider (e.g. `&[TvType::TvSeries, TvType::Movie]`)
    fn supported_types(&self) -> &[TvType];

    /// Returns a future which resolves into an anime info object (including the episodes). (*[`impl Future<Output = Result<IMovieInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L514-L529)*)\
    /// # Parameters
    /// * `media_id` - takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*)
    /// ```
    /// let movie_provider = movie::<provider_name>;
    /// let data = movie_provider.fetch_media_info(<media_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<Self::MediaInfo>;

    /// Returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L148-L153)*)\
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*
    /// ```
    /// let movie_provider = movie::<provider_name>;
    /// let data = movie_provider.fetch_episode_servers(<episode_id>, <media_id>).await??;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_episode_servers(
        &self,
        episode_id: String,
        media_id: String,
    ) -> anyhow::Result<Vec<Self::ServerResult>>;

    /// Returns a future which resolves into an vector of episode sources and subtitles. (*[`impl Future<Output = Result<ISource>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L406-L413)*)\
    /// # Parameters
    /// * `episode_id` - takes episode id as a parameter. (*episode id can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*)
    /// * `server (optional)` - [`StreamingServers`]
    /// ```
    /// let movie_provider = movie::<provider_name>;
    /// let data = movie_provider.fetch_episode_sources(<episode_id>, <media_id>, None).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_episode_sources(
        &self,
        episode_id: String,
        media_id: String,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<Self::SourceResult>;
}
