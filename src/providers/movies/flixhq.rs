use super::flixhq_html::{
    parse_episode_html, parse_info_html, parse_page_html, parse_recent_movie_html,
    parse_recent_shows_html, parse_search_html, parse_season_html, parse_server_html,
    parse_trending_movie_html, parse_trending_shows_html,
};
use crate::models::{
    BaseParser, BaseProvider, IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult,
    IMovieSeason, ISearch, ISource, MovieParser, StreamingServers, TvType,
};

use crate::extractors::{MixDrop, VidCloud};

use serde::Deserialize;

/// Contains all the FlixHQ Info
pub struct FlixHQ;

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    link: String,
}

#[derive(Debug)]
pub struct FlixHQInfo {
    pub base: IMovieResult,
    pub info: IMovieInfo,
}

impl BaseProvider for FlixHQ {
    #[inline]
    fn name(&self) -> &str {
        "FlixHQ"
    }

    #[inline]
    fn base_url(&self) -> &str {
        "https://flixhq.to"
    }

    #[inline]
    fn logo(&self) -> &str {
        "https://upload.wikimedia.org/wikipedia/commons/7/7a/MyAnimeList_Logo.png"
    }

    #[inline]
    fn class_path(&self) -> &str {
        "MOVIES.FlixHQ"
    }
}

impl BaseParser for FlixHQ {
    type BaseSearchResult = ISearch<IMovieResult>;

    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<Self::BaseSearchResult> {
        let page = page.unwrap_or(1);

        let url = format!("{}/search/{}?page={}", self.base_url(), query, page);
        let page_html = reqwest::Client::new().get(url).send().await?.text().await?;

        let (next_page, total_page, id) = parse_page_html(page_html)?;

        let mut results = vec![];

        for i in id.into_iter() {
            let result = self.fetch_search_results(i).await?;

            results.push(result);
        }

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: Some(next_page),
            total_pages: total_page,
            total_results: results.len(),
            results,
        })
    }
}

impl MovieParser for FlixHQ {
    type MediaInfo = FlixHQInfo;
    type ServerResult = IEpisodeServer;
    type SourceResult = ISource;

    #[inline]
    fn supported_types(&self) -> &[TvType] {
        &[TvType::Movie, TvType::TvSeries]
    }

    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<Self::MediaInfo> {
        self.fetch_info(media_id).await
    }

    async fn fetch_episode_servers(
        &self,
        episode_id: String,
        media_id: String,
    ) -> anyhow::Result<Vec<Self::ServerResult>> {
        self.fetch_servers(episode_id, media_id).await
    }

    async fn fetch_episode_sources(
        &self,
        episode_id: String,
        media_id: String,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<Self::SourceResult> {
        self.fetch_sources(episode_id, media_id, server).await
    }
}

impl FlixHQ {
    /// Returns a future which resolves into an movie result object (*[`impl Future<Output = Result<IMovieResult>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
    /// # Parameters
    /// * `id` - the id of a movie or show
    pub async fn fetch_search_results(&self, id: String) -> anyhow::Result<IMovieResult> {
        let url = format!("{}/{}", self.base_url(), id);

        let media_html = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        parse_search_html(media_html, id, url)
    }

    /// Returns a future which resolves into an movie info object (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/providers/movies/flixhq.rs#L22-L26)*)\
    /// # Parameters
    /// * `media_id` - takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*)
    pub async fn fetch_info(&self, media_id: String) -> anyhow::Result<FlixHQInfo> {
        let search_results = self.fetch_search_results(media_id.clone()).await?;

        let media_type = search_results.media_type.unwrap();
        let is_seasons = matches!(media_type, TvType::TvSeries);

        let info_html = reqwest::Client::new()
            .get(format!("{}/{}", self.base_url(), media_id))
            .send()
            .await?
            .text()
            .await?;

        let info = parse_info_html(info_html, search_results)?;

        if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = reqwest::Client::new()
                .get(format!("{}/ajax/v2/tv/seasons/{}", self.base_url(), id))
                .send()
                .await
                .unwrap()
                .text()
                .await?;

            let season_ids = parse_season_html(season_html)?;

            let mut seasons_and_episodes: Vec<Vec<IMovieEpisode>> = vec![];

            for (i, episode) in season_ids.iter().enumerate() {
                let episode_html = reqwest::Client::new()
                    .get(format!(
                        "{}/ajax/v2/season/episodes/{}",
                        self.base_url(),
                        &episode
                    ))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let episodes = parse_episode_html(self.base_url(), episode_html, i).unwrap();

                seasons_and_episodes.push(episodes);
            }

            Ok(FlixHQInfo {
                base: info.base,
                info: IMovieInfo {
                    total_episodes: seasons_and_episodes.last().map(|x| x.len()),
                    seasons: Some(IMovieSeason {
                        season: seasons_and_episodes
                            .last()
                            .and_then(|x| x.last())
                            .and_then(|y| y.season)
                            .unwrap_or(0),
                        image: None,
                        episodes: Some(seasons_and_episodes.clone()),
                    }),
                    episodes: Some(seasons_and_episodes),
                    ..info.info
                },
            })
        } else {
            Ok(FlixHQInfo {
                base: info.base,
                info: IMovieInfo {
                    total_episodes: None,
                    seasons: None,
                    episodes: None,
                    ..info.info
                },
            })
        }
    }

    /// Returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L135-L146)*)\
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*
    pub async fn fetch_servers(
        &self,
        episode_id: String,
        media_id: String,
    ) -> anyhow::Result<Vec<IEpisodeServer>> {
        let (episode_id, is_movie) = if !episode_id
            .starts_with(&format!("{}/ajax", self.base_url()))
            && !media_id.contains("movie")
        {
            (
                format!("{}/ajax/v2/episode/servers/{}", self.base_url(), episode_id),
                false,
            )
        } else {
            (
                format!("{}/ajax/movie/episodes/{}", self.base_url(), episode_id),
                true,
            )
        };

        let server_html = reqwest::Client::new()
            .get(episode_id)
            .send()
            .await?
            .text()
            .await?;

        let servers = parse_server_html(server_html, self.base_url(), is_movie, media_id)?;

        Ok(servers)
    }

    /// Returns a future which resolves into an vector of episode sources and subtitles. (*[`impl Future<Output = Result<ISource>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L374-L379)*)\
    /// # Parameters
    /// * `episode_id` - takes episode id as a parameter. (*episode id can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*)
    /// * `server (optional)` - [`StreamingServers`]
    pub async fn fetch_sources(
        &self,
        episode_id: String,
        media_id: String,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<ISource> {
        let server = server.unwrap_or(StreamingServers::UpCloud);
        let servers = self.fetch_servers(episode_id.clone(), media_id).await?;

        let i = servers
            .iter()
            .position(|s| s.name == server.to_string())
            .expect(&format!("Server {server} not found"));

        let parts: Vec<&str> = servers[i].url.split('.').collect();
        let server_id = parts.last().cloned().unwrap_or_default();

        let server_json = reqwest::Client::new()
            .get(format!("{}/ajax/get_link/{}", self.base_url(), server_id))
            .send()
            .await?
            .text()
            .await?;

        let server_info: ServerInfo =
            serde_json::from_str(&server_json).expect("Error parsing JSON");

        if server_info.link.starts_with("http") {
            match server {
                StreamingServers::MixDrop => {
                    let mut mix_drop = MixDrop {
                        sources: Vec::new(),
                        subtitles: Vec::new(),
                    };

                    mix_drop.extract(server_info.link.clone()).await?;

                    Ok(ISource {
                        sources: Some(mix_drop.sources),
                        subtitles: Some(mix_drop.subtitles),
                        headers: Some(server_info.link),
                        intro: None,
                    })
                }
                StreamingServers::VidCloud => {
                    let mut vid_cloud = VidCloud {
                        sources: Vec::new(),
                        subtitles: Vec::new(),
                    };

                    vid_cloud
                        .extract(server_info.link.clone(), Some(true))
                        .await
                        .expect("Failed to extract VidCloud sources!");

                    Ok(ISource {
                        sources: Some(vid_cloud.sources),
                        subtitles: Some(vid_cloud.subtitles),
                        headers: Some(server_info.link),
                        intro: None,
                    })
                }
                StreamingServers::UpCloud => {
                    let mut vid_cloud = VidCloud {
                        sources: Vec::new(),
                        subtitles: Vec::new(),
                    };

                    vid_cloud
                        .extract(server_info.link.clone(), None)
                        .await
                        .expect("Failed to extract UpCloud sources!");

                    Ok(ISource {
                        sources: Some(vid_cloud.sources),
                        subtitles: Some(vid_cloud.subtitles),
                        headers: Some(server_info.link),
                        intro: None,
                    })
                }
                _ => {
                    let mut vid_cloud = VidCloud {
                        sources: Vec::new(),
                        subtitles: Vec::new(),
                    };

                    vid_cloud
                        .extract(server_info.link.clone(), None)
                        .await
                        .expect("Failed to extract UpCloud sources!");

                    Ok(ISource {
                        sources: Some(vid_cloud.sources),
                        subtitles: Some(vid_cloud.subtitles),
                        headers: Some(server_info.link),
                        intro: None,
                    })
                }
            }
        } else {
            panic!("Incorrect server Url. Try Again.")
        }
    }

    /// Returns a future which resolves into an vector of movie results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn fetch_recent_movies(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let recent_movie_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
            .send()
            .await?
            .text()
            .await?;

        let id = parse_recent_movie_html(recent_movie_html)?;

        let mut results = vec![];

        for i in id.into_iter() {
            let result = self.fetch_search_results(i).await?;

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn fetch_recent_shows(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let recent_shows_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
            .send()
            .await?
            .text()
            .await?;

        let id = parse_recent_shows_html(recent_shows_html)?;

        let mut results = vec![];

        for i in id.into_iter() {
            let result = self.fetch_search_results(i).await?;

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of movie results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn fetch_trending_movies(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let trending_movies_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
            .send()
            .await?
            .text()
            .await?;

        let id = parse_trending_movie_html(trending_movies_html)?;

        let mut results = vec![];

        for i in id.into_iter() {
            let result = self.fetch_search_results(i).await?;

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn fetch_trending_shows(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let trending_shows_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
            .send()
            .await?
            .text()
            .await?;

        let id = parse_trending_shows_html(trending_shows_html)?;

        let mut results = vec![];

        for i in id.into_iter() {
            let result = self.fetch_search_results(i).await?;

            results.push(result);
        }

        Ok(results)
    }
}
