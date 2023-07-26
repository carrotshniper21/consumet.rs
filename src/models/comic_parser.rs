use crate::models::BaseParser;

/// A trait providing comic parsing methods to implement on
/// ```
/// use consumet_api_rs::models::LightNovelParser;
/// use consumet_api_rs::providers::comics;
///
/// // <provider_name> is the name of the provider you want to use.
/// let comic_provider = comics::<provider_name>;
/// ```
pub trait ComicParser: BaseParser {}
