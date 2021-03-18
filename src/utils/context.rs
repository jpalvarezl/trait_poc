use crate::cache::{CacheOuter, ServiceCache};

pub struct Context {
    pub cache: ServiceCache
}

impl Context {
    pub fn cache(&self) -> &impl CacheOuter {
        &self.cache
    }
}
