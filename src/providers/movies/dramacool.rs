use crate::extractors::{AsianLoad, MixDrop, StreamSB, StreamTape};
use crate::models::{
    BaseParser, BaseProvider, IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource,
    MovieParser, ProxyConfig, StreamingServers, TvType,
};

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
        todo!()
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

impl DramaCool {}
