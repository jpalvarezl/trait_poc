pub mod extensions;

pub struct ServiceCache {}

pub trait CacheOuter: private::CacheInner {}


// Using sealed trait pattern
mod private {
    pub trait CacheInner {
        fn get(&self);
        fn set(&self);
        fn info(&self) -> String;
    }

    impl CacheInner for super::ServiceCache {
        fn get(&self) {}

        fn set(&self) {}

        fn info(&self) -> String {
            String::from("impl CacheInner for ServiceCache")
        }
    }
}

impl CacheOuter for ServiceCache {}

