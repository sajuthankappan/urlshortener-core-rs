#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]

extern crate serde;
extern crate serde_json;

pub mod models;
pub mod errors;