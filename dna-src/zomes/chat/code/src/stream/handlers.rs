extern crate utils;
use hdk::error::ZomeApiResult;
use hdk::AGENT_ADDRESS;
use hdk::holochain_core_types::{
    hash::HashString,
    entry::Entry,
    cas::content::Address,
    json::RawString,
    link::LinkMatch,
};

use crate::stream::{
    Stream,
};

use utils::{
    GetLinksLoadResult,
    get_links_and_load_type
};
use crate::message;

pub fn handle_create_stream(
    name: String,
    description: String,
    initial_members: Vec<Address>,
) -> ZomeApiResult<Address> {

    let stream = Stream{name, description};

    let entry = Entry::App(
        "public_stream".into(),
        stream.into()
    );

    let stream_address = hdk::commit_entry(&entry)?;
    hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member", "", "")?;

    for member in initial_members {
        hdk::utils::link_entries_bidir(&member, &stream_address, "member_of", "has_member", "", "")?;
    }

    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_streams").into(),
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &stream_address, "public_stream", "")?;

    Ok(stream_address)
}

pub fn handle_join_stream(stream_address: HashString) -> ZomeApiResult<()> {
    hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member", "", "")?;
    Ok(())
}

pub fn handle_get_members(address: HashString) -> ZomeApiResult<Vec<Address>> {
    let all_member_ids = hdk::get_links(&address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    Ok(all_member_ids)
}

pub fn handle_get_messages(address: HashString) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
    get_links_and_load_type(&address, LinkMatch::Exactly("message_in"), LinkMatch::Any)
}

pub fn handle_post_message(stream_address: HashString, message_spec: message::MessageSpec) -> ZomeApiResult<()> {

    let message = message::Message::from_spec(
        &message_spec,
        &AGENT_ADDRESS.to_string());

    let message_entry = Entry::App(
        "message".into(),
        message.into(),
    );

    let message_addr = hdk::commit_entry(&message_entry)?;

    hdk::link_entries(&stream_address, &message_addr, "message_in", "")?;

    Ok(())
}

pub fn handle_get_all_public_streams() -> ZomeApiResult<Vec<GetLinksLoadResult<Stream>>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_streams").into(),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    get_links_and_load_type(&anchor_address, LinkMatch::Exactly("public_stream"), LinkMatch::Any)
}
