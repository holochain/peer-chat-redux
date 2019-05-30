
use hdk::error::ZomeApiResult;
use hdk::AGENT_ADDRESS;
use hdk::holochain_core_types::{
    hash::HashString,
    entry::Entry,
    cas::content::Address,
    json::RawString,
};

use crate::stream::{
    Stream,
};


use crate::utils;
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
    utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member")?;
    
    for member in initial_members {
        utils::link_entries_bidir(&member, &stream_address, "member_of", "has_member")?;
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
    utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member")?;
    Ok(())
}

pub fn handle_get_members(address: HashString) -> ZomeApiResult<Vec<Address>> {
    let all_member_ids = hdk::get_links(&address, Some("has_member".into()), None)?.addresses().to_owned();
    Ok(all_member_ids)
}

pub fn handle_get_messages(address: HashString) -> ZomeApiResult<utils::GetLinksLoadResult<message::Message>> {
    utils::get_links_and_load_type(&address, "message_in")
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

pub fn handle_get_all_public_streams() -> ZomeApiResult<utils::GetLinksLoadResult<Stream>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_streams").into(),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    utils::get_links_and_load_type(&anchor_address, "public_stream")
}
