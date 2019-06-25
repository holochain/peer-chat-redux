#![feature(try_from)]
extern crate hdk;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use hdk::holochain_json_api::{
    json::{default_to_json, JsonString},
};
use hdk::holochain_persistence_api::{
    cas::content::Address
};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLinksLoadResult<T> {
    pub entry: T,
    pub address: Address,
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetLinksLoadResult<T>> for JsonString {
    fn from(u: GetLinksLoadResult<T>) -> JsonString {
        default_to_json(u)
    }
}

// .map(|(address, entry)| GetLinksLoadResult { address, entry })