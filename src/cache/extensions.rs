use crate::cache::CacheOuter;
use std::fmt::Display;

pub trait OneDatabaseCache: CacheOuter {
    fn magic_method<R>(&self, input: R) where R: Display {
        println!("DB ID MAYBE: {}", input);
        println!("INFO: {}", self.info());
    }
}

impl<T: CacheOuter + ?Sized> OneDatabaseCache for T {}