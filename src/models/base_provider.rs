use crate::models::IProviderStats;

pub trait BaseProvider {
    fn name(&self) -> &str;

    fn base_url(&self) -> &str;

    #[inline]
    fn languages(&self) -> &[&str] {
        &["en"]
    }

    #[inline]
    fn is_nsfw(&self) -> bool {
        false
    }

    fn logo(&self) -> &str;

    fn class_path(&self) -> &str;

    #[inline]
    fn is_working(&self) -> bool {
        true
    }

    #[inline]
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
