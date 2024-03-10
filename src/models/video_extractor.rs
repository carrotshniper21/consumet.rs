use crate::models::ExtractConfig;

pub trait VideoExtractor {
    type VideoSource;

    /// takes video link
    /// returns video sources (video links) available
    fn extract(
        &mut self,
        video_url: String,
        args: ExtractConfig,
    ) -> impl std::future::Future<Output = anyhow::Result<Self::VideoSource>>;
}
