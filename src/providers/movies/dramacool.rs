use crate::{
    extractors::{
        mixdrop::{MixDropSource, MixDropSubtitle},
        AsianLoad, MixDrop, StreamSB, StreamTape,
    },
    html::movies::dramacool_html::DramaCoolHTML,
    models::{ExtractConfig, ISubtitle, IVideo, StreamingServers, VideoExtractor},
    CLIENT,
};

use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Contains all the DramaCool Info
pub struct DramaCool;

#[derive(Debug, Deserialize, Serialize)]
pub enum DramaCoolSourceType {
    AsianLoad(Vec<IVideo>),
    MixDrop(Vec<MixDropSource>),
    StreamTape(Vec<IVideo>),
    StreamSB(Vec<IVideo>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DramaCoolSubtitles {
    AsianLoad(Vec<ISubtitle>),
    MixDrop(Vec<MixDropSubtitle>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolSources {
    pub headers: Option<String>,
    pub subtitles: Option<DramaCoolSubtitles>,
    pub sources: DramaCoolSourceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolSearchResults {
    pub current_page: usize,
    pub has_next_page: bool,
    pub total_pages: usize,
    pub total_results: usize,
    pub results: Vec<DramaCoolResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub status: String,
    pub release_date: String,
    pub other_names: Vec<String>,
    pub description: String,
    pub genres: Vec<String>,
    pub country: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DramaCoolEpisode {
    pub id: String,
    pub title: String,
    pub sub_type: String,
    pub release_date: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DramaCoolInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub status: String,
    pub release_date: String,
    pub other_names: Vec<String>,
    pub description: String,
    pub genres: Vec<String>,
    pub country: Vec<String>,
    pub total_episodes: usize,
    pub episodes: Vec<DramaCoolEpisode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolServers {
    pub servers: Vec<DramaCoolServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DramaCoolServer {
    pub name: String,
    pub url: String,
}

pub(crate) const BASE_URL: &'static str = "https://dramacool.com.pa";

impl DramaCool {
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<DramaCoolSearchResults> {
        let current_page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = CLIENT
            .get(format!(
                "{}/search?keyword={}&page={}",
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

        let results: Arc<Mutex<Vec<DramaCoolResult>>> = Arc::new(Mutex::new(vec![]));

        bodies
            .for_each(|result| {
                let urls = urls.clone();
                let results = Arc::clone(&results);

                async move {
                    match result {
                        Ok((index, text)) => {
                            let url = &urls[index];
                            let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];

                            let result = self.single_page(text, id, url.to_string());

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

        Ok(DramaCoolSearchResults {
            current_page,
            has_next_page,
            total_pages,
            total_results: results.len(),
            results,
        })
    }

    pub async fn info(&self, media_id: &str) -> anyhow::Result<DramaCoolInfo> {
        let info_html = CLIENT
            .get(format!("{}/{}", BASE_URL, media_id))
            .send()
            .await?
            .text()
            .await?;

        let search_result = self.single_page(
            info_html.clone(),
            media_id,
            format!("{}/{}", BASE_URL, media_id),
        );

        let episodes = self.info_episode(info_html.clone());

        Ok(DramaCoolInfo {
            total_episodes: episodes.len(),
            episodes,
            id: search_result.id,
            title: search_result.title,
            url: search_result.url,
            image: search_result.image,
            status: search_result.status,
            release_date: search_result.release_date,
            other_names: search_result.other_names,
            description: search_result.description,
            genres: search_result.genres,
            country: search_result.country,
        })
    }

    pub async fn servers(&self, episode_id: &str) -> anyhow::Result<DramaCoolServers> {
        let server_html = CLIENT
            .get(format!("{}{}.html", BASE_URL, episode_id))
            .send()
            .await?
            .text()
            .await?;

        let servers = self.info_server(server_html);

        Ok(DramaCoolServers { servers })
    }

    pub async fn sources(
        &self,
        episode_id: &str,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<DramaCoolSources> {
        let server: StreamingServers = server.unwrap_or(StreamingServers::AsianLoad);

        let servers = self.servers(episode_id).await?;

        let i = match servers
            .servers
            .iter()
            .position(|s| s.name == server.to_string().to_lowercase())
        {
            Some(index) => index,
            None => panic!("Server not found!"),
        };

        let server_url = &servers.servers[i].url;

        match server {
            StreamingServers::AsianLoad => {
                let mut asianload = AsianLoad {
                    sources: vec![],
                    subtitles: vec![],
                };

                asianload
                    .extract(
                        server_url.to_string(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(DramaCoolSources {
                    sources: DramaCoolSourceType::AsianLoad(asianload.sources),
                    subtitles: Some(DramaCoolSubtitles::AsianLoad(asianload.subtitles)),
                    headers: None,
                })
            }
            StreamingServers::MixDrop => {
                let mut mix_drop = MixDrop {
                    sources: vec![],
                    subtitles: vec![],
                };

                mix_drop
                    .extract(
                        server_url.to_string(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(DramaCoolSources {
                    sources: DramaCoolSourceType::MixDrop(mix_drop.sources),
                    subtitles: Some(DramaCoolSubtitles::MixDrop(mix_drop.subtitles)),
                    headers: Some(server_url.to_string()),
                })
            }
            StreamingServers::StreamTape => {
                let mut streamtape = StreamTape { sources: vec![] };

                streamtape
                    .extract(
                        server_url.to_string(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(DramaCoolSources {
                    sources: DramaCoolSourceType::StreamTape(streamtape.sources),
                    subtitles: None,
                    headers: None,
                })
            }
            StreamingServers::StreamSB => {
                let mut streamsb = StreamSB { sources: vec![] };

                streamsb
                    .extract(
                        server_url.to_string(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(DramaCoolSources {
                    sources: DramaCoolSourceType::StreamSB(streamsb.sources),
                    subtitles: None,
                    headers: None,
                })
            }
            _ => {
                panic!("Please try a different server.")
            }
        }
    }
}
