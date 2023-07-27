use super::dramacool_html::{parse_page_html, parse_search_html};
use crate::models::{
    BaseParser, BaseProvider, IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource,
    MovieParser, ProxyConfig, StreamingServers, TvType,
};

use crate::extractors::{AsianLoad, MixDrop, StreamSB, StreamTape};

// Contains all the DramaCool Info
pub struct DramaCool;

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

        let re = regex::Regex::new(r"[\W_]+").unwrap();
        let result_query = re.replace_all(&query, "-");
        let url = format!(
            "{}/search?keyword={}&page={}",
            self.base_url(),
            result_query,
            page
        );
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

impl MovieParser for DramaCool {
    type MediaInfo = String;
    type ServerResult = String;
    type SourceResult = String;

    fn supported_types(&self) -> &[TvType] {
        todo!()
    }

    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<Self::MediaInfo> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        episode_id: String,
        media_id: String,
    ) -> anyhow::Result<Vec<Self::ServerResult>> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        episode_id: String,
        media_id: String,
        server: Option<StreamingServers>,
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

        parse_search_html(media_html, id, url)
    }
}
