use crate::models::BaseProvider;

/// Provides a search method to be implemented on
pub trait BaseParser: BaseProvider {
    type BaseSearchResult;

    /// # Parameters
    /// * `query` - query to search for. (*In this case, We're searching for `Vincenzo`*) P.S: `vincenzo` is a really good korean drama i highly recommend it.
    /// * `page` - page number (default: 1)
    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<Self::BaseSearchResult>;
}
