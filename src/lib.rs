// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

pub mod models;
pub mod db;

#[macro_use]
extern crate diesel;
extern crate dotenv;
