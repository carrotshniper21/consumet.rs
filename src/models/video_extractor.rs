use reqwest::Url;

use crate::models::{ISubtitle, IVideo};

pub enum Inum {
    IVideo(Vec<IVideo>),
    ISource,
    ISubtitle(Vec<ISubtitle>),
}

pub trait VideoExtractor<T> {
    /// # Arguments
    /// * `servername` - A string slice that holds the name of the server
    const SERVER_NAME: &'static str;

    /// Extracts the video from the videoUrl based on the implmentation
    async fn extract(&mut self, video_url: Url, args: T) -> anyhow::Result<Inum>;
}
