use super::dramacool_html::{create_html_fragment, Page, Search};
use crate::models::{
    BaseParser, BaseProvider, IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, ISearch,
    ISource, MovieParser, ProxyConfig, StreamingServers, TvType,
};

use crate::extractors::{AsianLoad, MixDrop, StreamSB, StreamTape};

use serde::Deserialize;

// Contains all the DramaCool Info
pub struct DramaCool;

#[derive(Debug, Deserialize)]
pub struct DramaCoolServerInfo {
    _link: String,
}

#[derive(Debug)]
pub struct DramaCoolInfo {
    pub base: IMovieResult,
    pub info: IMovieInfo,
}

impl BaseProvider for DramaCool {
    #[inline]
    fn name(&self) -> &str {
        "DramaCool"
    }

    #[inline]
    fn base_url(&self) -> &str {
        "https://dramacool.hr"
    }

    #[inline]
    fn logo(&self) -> &str {
        "https://play-lh.googleusercontent.com/IaCb2JXII0OV611MQ-wSA8v_SAs9XF6E3TMDiuxGGXo4wp9bI60GtDASIqdERSTO5XU"
    }

    #[inline]
    fn class_path(&self) -> &str {
        "MOVIES.DramaCool"
    }
}

impl BaseParser for DramaCool {
    type BaseSearchResult = ISearch<IMovieResult>;

    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<Self::BaseSearchResult> {
        let page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search?keyword={}&page={}",
                self.base_url(),
                parsed_query,
                page
            ))
            .send()
            .await?
            .text()
            .await?;

        let fragment = create_html_fragment(&page_html);

        let page_parser = Page { elements: fragment };

        let ids = page_parser.page_ids();

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let result = self.fetch_search_results(id.to_string()).await?;

            results.push(result);
        }

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: page_parser.has_next_page(),
            total_pages: page_parser.total_pages(),
            total_results: results.len(),
            results,
        })
    }
}

impl MovieParser for DramaCool {
    type MediaInfo = DramaCoolInfo;
    type ServerResult = String;
    type SourceResult = String;

    fn supported_types(&self) -> &[TvType] {
        todo!()
    }

    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<Self::MediaInfo> {
        self.fetch_info(media_id).await
    }

    async fn fetch_episode_servers(
        &self,
        _episode_id: String,
        _media_id: String,
    ) -> anyhow::Result<Vec<Self::ServerResult>> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _episode_id: String,
        _media_id: String,
        _server: Option<StreamingServers>,
    ) -> anyhow::Result<Self::SourceResult> {
        todo!()
    }
}

impl DramaCool {
    /// Returns a future which resolves into an movie result object (*[`impl Future<Output = Result<IMovieResult>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
    /// # Parameters
    /// * `id` - the id of the provided drama
    pub async fn fetch_search_results(&self, id: String) -> anyhow::Result<IMovieResult> {
        let url = format!("{}/{}", self.base_url(), id);

        let media_html = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        let fragment = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &fragment,
            id: &id,
        };

        Ok(IMovieResult {
            cover: None,
            title: search_parser.search_title(),
            other_names: search_parser.search_other_names(),
            url: Some(url),
            image: search_parser.search_image(),
            release_date: search_parser.search_release_date(),
            media_type: None,
            id: Some(id),
        })
    }

    pub async fn fetch_info(&self, media_id: String) -> anyhow::Result<DramaCoolInfo> {
        todo!()
    }
}
