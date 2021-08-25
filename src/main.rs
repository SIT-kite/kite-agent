#![allow(dead_code)]

// Jemallocator support.
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

mod agent;
mod config;
mod error;
mod net;
mod parser;
pub mod service;
mod zftool;

use agent::{run, SharedData};
use config::CONFIG;
use futures_util::core_reexport::time::Duration;
use net::SessionStorage;

fn worker_thread(storage: SessionStorage) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Fail to create runtime.");

    loop {
        let storage = storage.clone();

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
                                session: storage,
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
        std::thread::sleep(Duration::from_millis(30000));
    }
}

fn main() {
    let storage = SessionStorage::new().expect("Fail to load SessionStorage.");
    let mut worker_threads = Vec::new();

    for _ in 0..CONFIG.server.conn {
        let storage = storage.clone();

        let worker = std::thread::spawn(move || {
            worker_thread(storage);
        });
        worker_threads.push(worker);
    }

    loop {
        std::thread::sleep(Duration::from_millis(1000));
    }
}
