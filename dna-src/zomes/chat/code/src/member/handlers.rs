
use hdk::{
    AGENT_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        json::{RawString},
        cas::content::{Address, AddressableContent},

    },
    error::{
        ZomeApiResult,
        ZomeApiError,
    }
};

use crate::member::Profile;
use crate::utils;


pub fn get_my_member_id() -> Address {
    Address::from(AGENT_ADDRESS.to_string())
}

pub fn handle_register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("member_directory").into(),
    );

    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "member_tag")?;

    let profile_entry = Entry::App(
        "chat_profile".into(),
        Profile {
            name,
            avatar_url,
        }.into()
    );
    let profile_addr = hdk::commit_entry(&profile_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &profile_addr, "profile")?;

    Ok(AGENT_ADDRESS.to_string().into())
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    utils::get_links_and_load_type(&agent_address, "profile")?
        .iter()
        .next()
        .ok_or(ZomeApiError::Internal("Agent does not have a profile registered".into()))
        .map(|elem: &utils::GetLinksLoadElement<Profile>| {
            elem.entry.clone()
        })
}

pub fn handle_get_all_members() -> ZomeApiResult<Vec<Address>> {
    let anchor = Entry::App(
        "anchor".into(),
        RawString::from("member_directory").into(),
    );

    Ok(hdk::get_links(&anchor.address(), "member_tag")?.addresses().to_owned())
}