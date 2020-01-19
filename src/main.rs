#![feature(plugin, decl_macro, proc_macro_hygiene)]
// mute warnings, couse by compiler, should be fixed in rustc 1.4
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]
#![allow(dead_code)]

extern crate hex;
extern crate serde;
extern crate sodiumoxide;
extern crate ws;

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

#[macro_use]
extern crate log;

mod accounts_storage;
mod crypto;
mod database;
mod error;
mod primitives;
mod server;
mod ws_web_server;

use database::connection_pool;
use std::thread;
use std::time::Duration;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    crypto::init();

    // create connection pool to database
    let db_pool = connection_pool::init_pool();

    // run rest server!
    let rest_thread = thread::spawn(move || server::router::launch_routes(db_pool.clone()));
    println!("REST API created");

    // run ws server!
    let ws_server_thread =
        thread::spawn(move || ws_web_server::create_web_server("127.0.0.1:3012").unwrap());

    // Give the servers a little time to get going
    thread::sleep(Duration::from_millis(10));

    let _ = rest_thread.join();
    let _ = ws_server_thread.join();
}
