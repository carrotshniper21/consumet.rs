use crate::models::ExtractConfig;

pub trait VideoExtractor {
    type VideoSource;

    /// takes video link
    /// returns video sources (video links) available
    async fn extract(
        &mut self,
        video_url: String,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource>;
}
