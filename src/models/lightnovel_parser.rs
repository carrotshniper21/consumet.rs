use crate::models::BaseParser;
/// A trait providing light novel parsing methods to implement on
/// ```
/// use consumet_api_rs::models::LightNovelParser;
/// use consumet_api_rs::providers::light_novels;
///
/// // <provider_name> is the name of the provider you want to use.
/// let light_novel_provider = light_novels::<provider_name>;
/// ```
pub trait LightNovelParser: BaseParser {
    /// Returns a future which resolves into an light novel info object (including the chapters or volumes). (*[`impl Future<Output = Result<ILightNovelInfo>>`]("https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L324-L334)*)
    /// # Parameters
    /// * `light_novel_url` - id or url of the light novel. (*light novel id or url can be found in the light novel search results*)
    /// * `chapter_page` - chapter page number (*default: -1 meaning will fetch all chapters*)
    /// ```
    /// let light_novel_provider = light_novels::<provider_name>;
    /// let data = light_novel_provider.fetch_light_novel_info(<light_novel_info>, None).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_light_novel_info(
        &self,
        light_novel_url: String,
        chapter_page: Option<isize>,
    ) -> anyhow::Result<Box<dyn std::any::Any>>;

    /// Returns a content object. (*[`impl Future<Output = Result<ILightNovelChapterContent>>`]("https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L317-L322)*)
    /// # Parameters
    /// * `chapter_id` - chapter id. (*chapter id can be found in the light novel info object*)
    /// ```
    /// let light_novel_provider = light_novels::<provider_name>;
    /// let data = light_novel_provider.fetch_chapter_content(<chapter_id>).await?;
    /// println!("{:#?}", data);
    /// ```
    async fn fetch_chapter_content(
        &self,
        chapter_id: String,
    ) -> anyhow::Result<Box<dyn std::any::Any>>;
}
