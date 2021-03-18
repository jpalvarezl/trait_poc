pub mod extensions;

pub struct ServiceCache {}

pub trait CacheOuter: private::CacheInner {}


// Using sealed trait pattern
// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
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

