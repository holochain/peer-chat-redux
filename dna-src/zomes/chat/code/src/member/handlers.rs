
use hdk::{
    AGENT_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        json::{RawString},
        cas::content::{Address, AddressableContent},
    },
    error::{
        ZomeApiResult,
    }
};



pub fn get_my_member_id() -> Address {
    Address::from(AGENT_ADDRESS.to_string())
}

pub fn handle_get_all_members() -> ZomeApiResult<Vec<Address>> {
    let anchor = Entry::App(
        "anchor".into(),
        RawString::from("member_directory").into(),
    );

    Ok(hdk::get_links(&anchor.address(), "member_tag")?.addresses().to_owned())
}