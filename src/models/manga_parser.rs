use super::base_parser::BaseParser;
use super::types::{IMangaChapterPage, IMangaInfo};

use async_trait::async_trait;

#[async_trait]
pub trait MangaParser: BaseParser {
    async fn fetch_manga_info(
        &self,
        manga_id: String,
    ) -> Result<IMangaInfo, Box<dyn std::error::Error>>;
    async fn fetch_chapter_pages(
        &self,
        chapter_id: String,
    ) -> Result<Vec<IMangaChapterPage>, Box<dyn std::error::Error>>;
}
