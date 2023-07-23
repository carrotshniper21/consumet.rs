use crate::models::{BaseParser, IAnimeInfo, IEpisodeServer, ISource};

pub trait AnimeParser: BaseParser {
    fn is_dub_available_separately(&self) -> bool;

    async fn fetch_anime_info(&self, anime_id: String) -> anyhow::Result<IAnimeInfo>;

    async fn fetch_episode_sources(&self, episode_id: String) -> anyhow::Result<ISource>;

    async fn fetch_episode_servers(
        &self,
        episode_id: String,
    ) -> anyhow::Result<Vec<IEpisodeServer>>;
}
