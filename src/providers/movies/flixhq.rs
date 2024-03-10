use crate::{
    extractors::{
        MixDrop, MixDropSource, MixDropSubtitle, VidCloud, VidCloudSource, VidCloudSubtitle,
    },
    html::movies::flixhq_html::FlixHQHTML,
    models::{ExtractConfig, StreamingServers, TvType, VideoExtractor},
    CLIENT,
};

use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Contains all the FlixHQ Info
pub struct FlixHQ;

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQSourceType {
    VidCloud(Vec<VidCloudSource>),
    MixDrop(Vec<MixDropSource>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQSubtitles {
    VidCloud(Vec<VidCloudSubtitle>),
    MixDrop(Vec<MixDropSubtitle>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQSources {
    pub headers: String,
    pub subtitles: FlixHQSubtitles,
    pub sources: FlixHQSourceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServers {
    pub servers: Vec<FlixHQServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServer {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlixHQSeason {
    pub total_seasons: usize,
    pub episodes: Vec<Vec<FlixHQEpisode>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlixHQEpisode {
    pub id: String,
    pub title: String,
    pub url: String,
}

/// Contains Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQSearchResults {
    pub current_page: usize,
    pub has_next_page: bool,
    pub total_pages: usize,
    pub total_results: usize,
    pub results: Vec<FlixHQResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlixHQResult {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub media_type: TvType,
    pub genres: Vec<String>,
    pub description: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub country: Vec<String>,
    pub production: Vec<String>,
    pub casts: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQInfo {
    Tv(FlixHQShow),
    Movie(FlixHQMovie),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQMovie {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub media_type: TvType,
    pub genres: Vec<String>,
    pub description: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub country: Vec<String>,
    pub production: Vec<String>,
    pub casts: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQShow {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub media_type: TvType,
    pub genres: Vec<String>,
    pub description: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub country: Vec<String>,
    pub production: Vec<String>,
    pub casts: Vec<String>,
    pub tags: Vec<String>,
    pub total_episodes: usize,
    pub seasons: FlixHQSeason,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServerInfo {
    link: String,
}

pub const BASE_URL: &'static str = "https://flixhq.to";

impl FlixHQ {
    /// Returns a future which resolves into FlixHQSearchResults. (*[`impl Future<Output = Result<FlixHQSearchResults>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L60-L68)*)\
    /// # Parameters
    /// `query` - query to search for. (*In this case, We're searching for `Vincenzo`*) P.S: `vincenzo` is a really good korean drama i highly recommend it. |
    /// `page (optional)` - page number (default: 1)                                                                                                                   |
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<FlixHQSearchResults> {
        let current_page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = CLIENT
            .get(format!(
                "{}/search/{}?page={}",
                BASE_URL, parsed_query, current_page
            ))
            .send()
            .await?
            .text()
            .await?;

        let (ids, has_next_page, total_pages) = self.parse_search(page_html);

        let mut urls = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            urls.push(url);
        }

        let bodies = stream::iter(urls.clone())
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await.map(|text| (index, text))
                }
            })
            .buffer_unordered(urls.len());

        let results: Arc<Mutex<Vec<FlixHQResult>>> = Arc::new(Mutex::new(vec![]));

        bodies
            .for_each(|result| {
                let urls = urls.clone(); // Clone urls again for each closure
                let results = Arc::clone(&results);
                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                            let result = self.single_page(text, id, url.to_string()); // Assuming single_page function is defined somewhere
                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(FlixHQSearchResults {
            current_page,
            has_next_page,
            total_pages,
            total_results: results.len(),
            results,
        })
    }

    /// Returns a future which resolves into an enum containing extra media info (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L90-L94)*)\
    /// # Parameters
    /// * `media_id` - takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*)
    pub async fn info(&self, media_id: &str) -> anyhow::Result<FlixHQInfo> {
        let info_html = CLIENT
            .get(format!("{}/{}", BASE_URL, media_id))
            .send()
            .await?
            .text()
            .await?;

        let search_result =
            self.single_page(info_html, media_id, format!("{}/{}", BASE_URL, media_id));

        let media_type = search_result.media_type;
        let is_seasons = matches!(media_type, TvType::TvSeries);

        if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = CLIENT
                .get(format!("{}/ajax/v2/tv/seasons/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let season_ids = self.info_season(season_html);

            let mut seasons_and_episodes = vec![];

            for (i, episode) in season_ids.iter().enumerate() {
                let episode_html = CLIENT
                    .get(format!("{}/ajax/v2/season/episodes/{}", BASE_URL, &episode))
                    .send()
                    .await?
                    .text()
                    .await?;

                let episodes = self.info_episode(episode_html, i);
                seasons_and_episodes.push(episodes.episodes);
            }

            Ok(FlixHQInfo::Tv(FlixHQShow {
                total_episodes: seasons_and_episodes.last().map(|x| x.len()).unwrap(),
                seasons: FlixHQSeason {
                    total_seasons: seasons_and_episodes.len(),
                    episodes: seasons_and_episodes.clone(),
                },
                id: search_result.id,
                cover: search_result.cover,
                title: search_result.title,
                url: search_result.url,
                image: search_result.image,
                release_date: search_result.release_date,
                media_type: search_result.media_type,
                genres: search_result.genres,
                description: search_result.description,
                rating: search_result.rating,
                quality: search_result.quality,
                duration: search_result.duration,
                country: search_result.country,
                production: search_result.production,
                casts: search_result.casts,
                tags: search_result.tags,
            }))
        } else {
            Ok(FlixHQInfo::Movie(FlixHQMovie {
                id: search_result.id,
                cover: search_result.cover,
                title: search_result.title,
                url: search_result.url,
                image: search_result.image,
                release_date: search_result.release_date,
                media_type: search_result.media_type,
                genres: search_result.genres,
                description: search_result.description,
                rating: search_result.rating,
                quality: search_result.quality,
                duration: search_result.duration,
                country: search_result.country,
                production: search_result.production,
                casts: search_result.casts,
                tags: search_result.tags,
            }))
        }
    }

    /// Returns a future which resolves into FlixHQServers (*[`impl Future<Output = Result<FlixHQServers>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L36-L39)*)\
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*
    pub async fn servers(&self, episode_id: &str, media_id: &str) -> anyhow::Result<FlixHQServers> {
        let episode_id = format!(
            "{}/ajax/{}",
            BASE_URL,
            if !episode_id.starts_with(&format!("{}/ajax", BASE_URL)) && !media_id.contains("movie")
            {
                format!("v2/episode/servers/{}", episode_id)
            } else {
                format!("movie/episodes/{}", episode_id)
            }
        );

        let server_html = CLIENT.get(episode_id).send().await?.text().await?;

        let servers = self.info_server(server_html, media_id);

        Ok(FlixHQServers { servers })
    }

    /// Returns a future which resolves into FlixHQSources. (*[`impl Future<Output = Result<FlixHQSources>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L29-L34*)\
    /// # Parameters
    /// * `episode_id` - takes episode id as a parameter. (*episode id can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*)
    /// * `server (optional)` - [`StreamingServers`](https://github.com/eatmynerds/consumet.rs/blob/master/src/models/types.rs#L185-L198) | takes server enum as a parameter. *default: [`StreamingServers::VidCloud`](https://github.com/consumet-rs/consumet.rs/blob/master/src/models/types.rs#L177)
    pub async fn sources(
        &self,
        episode_id: &str,
        media_id: &str,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<FlixHQSources> {
        let server: StreamingServers = server.unwrap_or(StreamingServers::UpCloud);
        let servers = self.servers(episode_id, media_id).await?;

        let i = match servers
            .servers
            .iter()
            .position(|s| s.name == server.to_string())
        {
            Some(index) => index,
            None => 0,
        };

        let parts = &servers.servers[i].url;

        let server_id: &str = parts
            .split('.')
            .collect::<Vec<_>>()
            .last()
            .copied()
            .unwrap_or_default();

        let server_json = CLIENT
            .get(format!("{}/ajax/get_link/{}", BASE_URL, server_id))
            .send()
            .await?
            .text()
            .await?;

        let server_info: FlixHQServerInfo = serde_json::from_str(&server_json)?;

        match server {
            StreamingServers::MixDrop => {
                let mut mix_drop = MixDrop {
                    sources: vec![],
                    subtitles: vec![],
                };

                mix_drop
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSources {
                    sources: FlixHQSourceType::MixDrop(mix_drop.sources),
                    subtitles: FlixHQSubtitles::MixDrop(mix_drop.subtitles),
                    headers: server_info.link,
                })
            }
            StreamingServers::VidCloud => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            is_alternative: Some(true),
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSources {
                    sources: FlixHQSourceType::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
            StreamingServers::UpCloud => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSources {
                    sources: FlixHQSourceType::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
            _ => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            is_alternative: Some(false),
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSources {
                    sources: FlixHQSourceType::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
        }
    }

    /// Returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
    /// # Parameters
    /// * `None`
    pub async fn recent_movies(&self) -> anyhow::Result<Vec<FlixHQResult>> {
        let recent_html = CLIENT
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_recent_movies(recent_html);

        let mut urls = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            urls.push(url);
        }

        let bodies = stream::iter(urls.clone())
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await.map(|text| (index, text))
                }
            })
            .buffer_unordered(urls.len());

        let results: Arc<Mutex<Vec<FlixHQResult>>> = Arc::new(Mutex::new(Vec::new()));

        bodies
            .for_each(|result| {
                let urls = urls.clone(); // Clone urls again for each closure
                let results = Arc::clone(&results);
                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                            let result = self.single_page(text, id, url.to_string()); // Assuming single_page function is defined somewhere
                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
    /// # Parameters
    /// * `None`
    pub async fn recent_shows(&self) -> anyhow::Result<Vec<FlixHQResult>> {
        let recent_html = CLIENT
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_recent_shows(recent_html);

        let mut urls = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            urls.push(url);
        }

        let bodies = stream::iter(urls.clone())
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await.map(|text| (index, text))
                }
            })
            .buffer_unordered(urls.len());

        let results: Arc<Mutex<Vec<FlixHQResult>>> = Arc::new(Mutex::new(Vec::new()));

        bodies
            .for_each(|result| {
                let urls = urls.clone(); // Clone urls again for each closure
                let results = Arc::clone(&results);
                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                            let result = self.single_page(text, id, url.to_string()); // Assuming single_page function is defined somewhere
                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(results)
    }

    /// Returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
    /// # Parameters
    /// * `None`
    pub async fn trending_movies(&self) -> anyhow::Result<Vec<FlixHQResult>> {
        let trending_html = CLIENT
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_trending_movies(trending_html);

        let mut urls = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            urls.push(url);
        }

        let bodies = stream::iter(urls.clone())
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await.map(|text| (index, text))
                }
            })
            .buffer_unordered(urls.len());

        let results: Arc<Mutex<Vec<FlixHQResult>>> = Arc::new(Mutex::new(Vec::new()));

        bodies
            .for_each(|result| {
                let urls = urls.clone(); // Clone urls again for each closure
                let results = Arc::clone(&results);
                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                            let result = self.single_page(text, id, url.to_string()); // Assuming single_page function is defined somewhere
                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
    /// # Parameters
    /// * `None`
    pub async fn trending_shows(&self) -> anyhow::Result<Vec<FlixHQResult>> {
        let trending_html = CLIENT
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_trending_shows(trending_html);

        let mut urls = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            urls.push(url);
        }

        let bodies = stream::iter(urls.clone())
            .enumerate()
            .map(|(index, url)| {
                let client = &CLIENT;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await.map(|text| (index, text))
                }
            })
            .buffer_unordered(urls.len());

        let results: Arc<Mutex<Vec<FlixHQResult>>> = Arc::new(Mutex::new(Vec::new()));

        bodies
            .for_each(|result| {
                let urls = urls.clone(); // Clone urls again for each closure
                let results = Arc::clone(&results);
                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                            let result = self.single_page(text, id, url.to_string()); // Assuming single_page function is defined somewhere
                            results.lock().unwrap().push(result);
                        }
                        Err(err) => {
                            eprintln!("Error processing url: {}", err);
                        }
                    }
                }
            })
            .await;

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

        Ok(results)
    }
}
