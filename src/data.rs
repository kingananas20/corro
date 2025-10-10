use crate::cache;
use docsrs::{Doc, Indexed};
use log::info;

pub struct Data {
    pub playground_client: playground_api::Client,
    pub redis_client: cache::Cache,
    pub crates_io_client: crates_io_api::AsyncClient,
    pub max_code_size: u32,
    pub std: Doc<Indexed>,
    pub core: Doc<Indexed>,
    pub alloc: Doc<Indexed>,
}

impl Default for Data {
    fn default() -> Self {
        let email = std::env::var("EMAIL").expect("no email specified in the environment");
        let redis_url = std::env::var("REDIS_URL").expect("no redis_url specified");

        info!("reading, parsing and building searchindex for std.json");
        let std = Doc::from_json("./assets/docs/std.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        info!("reading, parsing and building searchindex for core.json");
        let core = Doc::from_json("./assets/docs/core.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        info!("reading, parsing and building searchindex for alloc.json");
        let alloc = Doc::from_json("./assets/docs/alloc.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        Self {
            playground_client: playground_api::Client::default(),
            redis_client: cache::Cache::new(&redis_url).unwrap(),
            crates_io_client: crates_io_api::AsyncClient::new(
                &format!("corro-discord-bot ({email})"),
                std::time::Duration::from_millis(1000),
            )
            .expect("failed to create an AsyncClient"),
            max_code_size: 64 * 1024,
            std,
            core,
            alloc,
        }
    }
}
