use crate::models::{ISubtitle, IVideo};
use serde::Deserialize;

/// Contains all the possible variants of what a extractor can return (so far)
#[derive(Debug, Deserialize)]
pub enum Inum {
    IVideo(Vec<IVideo>),
    ISource,
    ISubtitle(Vec<ISubtitle>),
}

/// Trait that contains methods for extractors to use
pub trait VideoExtractor<T> {
    /// # Arguments
    /// * `servername` - A string slice that holds the name of the server
    const SERVER_NAME: &'static str;

    /// Extracts the sources from the video_url
    /// # Parameters
    /// * `video_url` - A string that contains the server embed url
    async fn extract(&mut self, video_url: String, args: T) -> anyhow::Result<Inum>;
}
