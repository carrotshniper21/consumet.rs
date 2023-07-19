use super::base_parser::BaseParser;

use async_trait::async_trait;

#[async_trait]
pub trait LightNovelParser: BaseParser {
    async fn fetch_light_novel_info(
        &self,
        light_novel_url: String,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;

    async fn fetch_chapter_content(
        &self,
        chapter_id: String,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;
}
