use crate::utils::context::Context;
use crate::cache::ServiceCache;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};

mod cache;
mod providers;
mod utils;

fn main() {
    let context = Context{ cache: ServiceCache {} };

    let info = DefaultInfoProvider::new(&context);
    info.get_token_info();
    println!("Hello, world!");
}
