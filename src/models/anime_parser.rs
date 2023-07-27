use crate::models::{BaseParser, IAnimeInfo, IEpisodeServer, ISource};

/// A trait providing anime parsing methods to implement on
/// ```
/// use consumet_api_rs::models::AnimeParser;
/// use consumet_api_rs::providers::anime;
///
/// // <provider_name> is the name of the provider you want to use.
/// let anime_provider = anime::<provider_name>;
/// ```
pub trait AnimeParser: BaseParser {
    /// Returns a bool for whether or not the dub is seperate.
    fn is_dub_available_separately(&self) -> bool;

    /// Returns a future which resolves into an anime info object (including the episodes). (*[`impl Future<Output = Result<IAnimeInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L110-L133)*)
    /// # Parameters
    /// * `id` - takes anime id as a parameter. (*anime id can be found in the anime search results or anime info object*)
    /// ```
    /// let anime_provider = anime::<provider_name>;
    /// let data = anime_provider.fetch_anime_info(<anime_id>)
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_anime_info(&self, anime_id: String) -> anyhow::Result<IAnimeInfo>;

    /// Returns a future which resolves into an vector of episode sources. (*[`impl Future< Output = Result<ISource>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L406-L413)*)
    /// # Parameters
    /// * `episode_id` - takes episode id as parameter. (*episode id can be found in the anime info object*)
    /// ```
    /// let anime_provider = anime::<provider_name>;
    /// let data = anime_provider.fetch_episode_sources(<episode_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_episode_sources(&self, episode_id: String) -> anyhow::Result<ISource>;

    /// Returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L148-L153)*)
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the anime info object*)
    /// ```
    /// let anime_provider = anime::<provider_name>;
    /// let data = anime_provider.fetch_episode_sservers(<episode_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_episode_servers(
        &self,
        episode_id: String,
    ) -> anyhow::Result<Vec<IEpisodeServer>>;
}
