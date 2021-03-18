pub mod extensions;

pub struct ServiceCache {}

pub trait CacheOuter: CacheInner {}

trait CacheInner {
    fn get(&self);
    fn set(&self);
    fn info(&self) -> String;
}

impl CacheOuter for ServiceCache {}

impl CacheInner for ServiceCache {
    fn get(&self) {}

    fn set(&self) {}

    fn info(&self) -> String {
        String::from("impl CacheInner for ServiceCache")
    }
}