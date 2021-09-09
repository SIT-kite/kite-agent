#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

// Jemallocator support.
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;
use tokio::time::Duration;

use agent::{run, SharedData};
use config::CONFIG;
use net::SessionStorage;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod agent;
mod config;
mod error;
mod net;
mod parser;
pub mod service;

fn worker_thread(storage: SessionStorage, client: reqwest::Client) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Fail to create runtime.");

    loop {
        let storage = storage.clone();
        let client = client.clone();

        // Run on current thread.
        runtime.block_on(async move {
            let remote_server = &CONFIG.server.addr;
            let node_name = &CONFIG.agent.name;

            let local = tokio::task::LocalSet::new();

            // Run the local task set.
            local
                .run_until(async move {
                    tokio::task::spawn_local(async move {
                        run(
                            remote_server.clone(),
                            SharedData {
                                node: node_name.clone(),
                                session_store: storage,
                                client,
                            },
                        )
                        .await
                        .unwrap_or_else(|e| eprintln!("{}", e));
                    })
                    .await;
                })
                .await;
            /* KiteService has been aborted now.*/
        });

        println!("Trying to reconnect...");
        std::thread::sleep(Duration::from_secs(10));
    }
}

fn main() {
    let mut builder = reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none());

    if let Some(proxy) = &CONFIG.agent.proxy {
        let err_msg = "Invalid proxy settings.";
        builder = builder
            .proxy(reqwest::Proxy::http(proxy).expect(err_msg))
            .proxy(reqwest::Proxy::https(proxy).expect(err_msg))
            .danger_accept_invalid_certs(true);

        println!("Load proxy: {}", proxy);
    }
    let http_client = builder.build().expect("Could not init http client.");
    let storage = SessionStorage::new().expect("Fail to load SessionStorage.");
    let mut worker_threads = Vec::new();

    for _ in 0..CONFIG.server.conn {
        let client = http_client.clone();
        let storage = storage.clone();

        let worker = std::thread::spawn(move || {
            worker_thread(storage, client);
        });
        worker_threads.push(worker);
    }

    loop {
        std::thread::sleep(Duration::from_millis(1000));
    }
}
