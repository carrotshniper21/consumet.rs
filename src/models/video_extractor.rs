pub trait VideoExtractor<T> {
    type VideoSource;

    /// takes video link
    /// returns video sources (video links) available
    async fn extract(
        &self,
        video_url: String,
        bofadeez: &mut Vec<T>,
    ) -> anyhow::Result<Self::VideoSource>;
}
