use crate::models::IProviderStats;

/// Used to initalize a provider
pub trait BaseProvider {
    /// * `name` - name of the current provider
    fn name(&self) -> &str;

    /// * `base_url` - url to the base URL of the current provider
    fn base_url(&self) -> &str;

    #[inline]
    /// * `languages` - the language of the current provider, return language code, default: `["en"]`
    fn languages(&self) -> &[&str] {
        &["en"]
    }

    #[inline]
    /// * `is_nsfw` - if the provider providers NSFW content, default: `false`
    fn is_nsfw(&self) -> bool {
        false
    }

    /// * `logo` - url to the logo image of the current provider
    fn logo(&self) -> &str;

    /// * `class_path` - string
    fn class_path(&self) -> &str;

    /// * `is_working` - a bool to identify the state of the current provider, `true` if the provider is working, `false` otherwise. default: `true`
    #[inline]
    fn is_working(&self) -> bool {
        true
    }

    #[inline]
    /// Returns provider stats (*[`IProviderStats`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L11-L21)*)
    fn get_stats(&self) -> IProviderStats {
        IProviderStats {
            name: self.name(),
            base_url: self.base_url(),
            lang: self.languages(),
            is_nsfw: self.is_nsfw(),
            logo: self.logo(),
            class_path: self.class_path(),
            is_working: self.is_working(),
        }
    }
}
