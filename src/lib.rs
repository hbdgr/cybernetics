#![feature(plugin, decl_macro, proc_macro_hygiene)]
// mute warnings, couse by compiler, should be fixed in rustc 1.4
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

extern crate serde;
extern crate sodiumoxide;
extern crate ws;

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

#[macro_use]
extern crate log;

mod accounts_storage;
pub mod crypto;
mod database;
mod error;
mod primitives;
pub mod server;
mod ws_web_server;
