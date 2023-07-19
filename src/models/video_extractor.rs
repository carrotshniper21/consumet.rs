use async_trait::async_trait;
use reqwest::Url;

use super::types::{ISubtitle, IVideo};

pub enum Inum {
    IVideo(Vec<IVideo>),
    ISource,
    ISubtitle(Vec<ISubtitle>),
}

#[async_trait]
pub trait VideoExtractor<T> {
    /// # Arguments
    /// * `servername` - A string slice that holds the name of the server
    const SERVER_NAME: &'static str;

    /// // Extracts the video from the videoUrl based on the implmentation
    async fn extract(
        &mut self,
        video_url: Url,
        args: T,
    ) -> Result<Inum, Box<dyn std::error::Error>>;
}

/*
mod models;
use models::video_extractor::VideoExtractor;
use models::video_extractor::Inum;

struct Mixdrop {
    sources: Vec<IVideo>
}

impl Mixdrop {
    fn new() -> Self {
        Self {
            url
            ism3u8
            poster
        }
    }
}

impl VideoExtrator for Mixdrop {
    const serverName: &str = "mixdrop"

    async fn extract(&mut self, videoUrl: Url) -> Result<Inum, Box<dyn std::error::Error>>  {
        source = Ivideo{url: link, i33u8: bool, poster: link};

        &self.sources.push(
            source,
        )

        Ok(Inum::IVideo(&self.sources))
    }
}
*/

/*
impl VideoExtractor for (class name) {
    const serverName: &str = "server"

    fn get_sources(&self) -> &[IVideo] {
        self.sources
    }

    async fn extract(&self, videoUrl: Url) -> Result<Inum, Box<dyn std::error::Error>>  {

    }
}
*/

/*
somefile.rs

mod extractors;
use extractors::mixdrop::Mixdrop;

let something = Mixdrop::new();
something.extract();

*/
