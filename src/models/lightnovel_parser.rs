use crate::models::BaseParser;

pub trait LightNovelParser: BaseParser {
    async fn fetch_light_novel_info(
        &self,
        light_novel_url: String,
    ) -> anyhow::Result<Box<dyn std::any::Any>>;

    async fn fetch_chapter_content(
        &self,
        chapter_id: String,
    ) -> anyhow::Result<Box<dyn std::any::Any>>;
}
