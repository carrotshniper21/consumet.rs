use crate::models::{BaseParser, IMangaChapterPage, IMangaInfo};

/// A trait providing manga parsing methods to implement on
/// ```
/// use consumet_api_rs::models::MangaParser;
/// use consumet_api_rs::providers::manga;
///
/// // <provider_name> is the name of the provider you want to use.
/// let manga_provider = manga::<provider_name>;
/// ```
pub trait MangaParser: BaseParser {
    /// Returns a future which resolves into an manga info object (including the chapters). (*[`impl Future<Output = Result<IMangaInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L280-L290)*)\
    /// # Parameters
    /// * `manga_id` - manga id.(*manga id can be found in the manga search results*)
    /// ```
    /// let manga_provider = manga::<provider_name>;
    /// let data = manga_provider.fetch_manga_info(<manga_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_manga_info(&self, manga_id: String) -> anyhow::Result<IMangaInfo>;

    /// Returns a future which resolves into an vector of pages. (*[`impl Future<Output = Result<Vec<IMangaChapterPage>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L292-L297)*)\
    /// # Parameters
    /// * `chapter_id` - chapter id.(*chapter id can be found in the manga info*)
    /// ```
    /// let manga_provider = manga::<provider_name>;
    /// let data = manga_provider.fetch_manga_info(<chapter_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_chapter_pages(
        &self,
        chapter_id: String,
    ) -> anyhow::Result<Vec<IMangaChapterPage>>;
}
