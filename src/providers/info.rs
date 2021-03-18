use crate::cache::CacheOuter;
use crate::cache::extensions::OneDatabaseCache;
use crate::utils::context::Context;

pub trait InfoProvider {
    fn get_token_info(&self);
    fn get_address_info(&self);
    fn get_safe_info(&self);
}

pub struct DefaultInfoProvider<'p> {
    pub cache: &'p dyn CacheOuter
}

impl InfoProvider for DefaultInfoProvider<'_> {
    fn get_token_info(&self) {
        self.cache.magic_method(1);
    }
    fn get_address_info(&self) {}
    fn get_safe_info(&self) {}
}


impl DefaultInfoProvider<'_> {
    pub fn new(context: &Context) -> DefaultInfoProvider {
        DefaultInfoProvider {
            cache: context.cache()
        }
    }
}
