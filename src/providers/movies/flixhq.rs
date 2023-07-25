use super::flixhq_html::{
    parse_episode_html, parse_info_html, parse_page_html, parse_search_html, parse_season_html,
    parse_server_html,
};
use crate::models::{
    BaseParser, BaseProvider, IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult,
    IMovieSeason, ISearch, ISource, MovieParser, StreamingServers, TvType,
};

use crate::extractors::VidCloud;

use serde::Deserialize;
use std::future::Future;

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

impl BaseParser for FlixHQ {}

impl MovieParser for FlixHQ {
    type SearchResult = IMovieResult;
    type MediaInfo = FlixHQInfo;
    type ServerResult = IEpisodeServer;
    type SourceResult = ISource;

    #[inline]
    fn supported_types(&self) -> &[TvType] {
        &[TvType::Movie, TvType::TvSeries]
    }

    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<Self::SearchResult>> {
        let page = page.unwrap_or(1);
        let fetch = self.fetch_search_results();

        let url = format!("{}/search/{}?page={}", self.base_url(), query, page);
        let page_html = reqwest::Client::new().get(url).send().await?.text().await?;

        let (next_page, total_page, id) = parse_page_html(page_html)?;

        // NOTE: `Vec<impl Future<Output = Result<IMovieResult>>`
        let tasks = id
            .into_iter()
            .map(|id| {
                let fetched = fetch(id.clone());
                async move {
                    Box::into_pin(fetched)
                        .await
                        .map_err(|err| anyhow::anyhow!("Err: Can't fetch {}, {}", id, err))
                }
            })
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(tasks).await?;

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: Some(next_page),
            total_pages: total_page,
            total_results: results.len(),
            results,
        })
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
    pub fn fetch_search_results(
        &self,
    ) -> impl Fn(String) -> Box<dyn Future<Output = anyhow::Result<IMovieResult>> + Send> {
        let base_url = self.base_url().to_owned();

        move |id| {
            let base_url = base_url.clone();

            Box::new(async move {
                let url = format!("{}/{}", base_url, id);

                let media_html = reqwest::Client::new()
                    .get(&url)
                    .send()
                    .await?
                    .text()
                    .await?;

                parse_search_html(media_html, id, url)
            })
        }
    }

    pub async fn fetch_info(&self, media_id: String) -> anyhow::Result<FlixHQInfo> {
        let search_results = Box::into_pin(self.fetch_search_results()(media_id.clone())).await?;

        let media_type = search_results.media_type.unwrap();
        let is_seasons = matches!(media_type, TvType::TvSeries);

        let info_html = reqwest::Client::new()
            .get(format!("{}/{}", self.base_url(), media_id))
            .send()
            .await?
            .text()
            .await?;

        let title = search_results.title.clone();
        let info = parse_info_html(info_html, search_results)?;

        let seasons_and_episodes = if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = reqwest::Client::new()
                .get(format!("{}/ajax/v2/tv/seasons/{}", self.base_url(), id))
                .send()
                .await
                .unwrap()
                .text()
                .await?;

            let season_ids = parse_season_html(season_html)?;

            let mut episode_futures = vec![];

            for (i, episode) in season_ids.iter().enumerate() {
                let episode_future = async move {
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

                    episodes
                };

                episode_futures.push(episode_future);
            }

            futures::future::join_all(episode_futures).await
        } else {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();
            let episodes = IMovieEpisode {
                url: Some(format!("{}/ajax/movie/episodes/{}", self.base_url(), id)),
                id,
                title: format!("{} Movie", title.unwrap()),
                number: None,
                season: None,
                description: None,
                image: None,
                release_date: None,
            };

            vec![vec![episodes]]
        };

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
    }

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
                    todo!()
                }
                StreamingServers::VidCloud => {
                    let mut vid_cloud = VidCloud {
                        sources: Vec::new(),
                        subtitles: Vec::new(),
                    };

                    vid_cloud
                        .extract(server_info.link.clone(), Some(true))
                        .await?;
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

                    vid_cloud.extract(server_info.link.clone(), None).await?;
                    Ok(ISource {
                        sources: Some(vid_cloud.sources),
                        subtitles: Some(vid_cloud.subtitles),
                        headers: Some(server_info.link),
                        intro: None,
                    })
                }
                _ => {
                    todo!()
                }
            }
        } else {
            panic!("Incorrect server url")
        }
    }
}
