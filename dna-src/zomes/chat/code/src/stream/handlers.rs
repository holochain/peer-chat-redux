
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::{
    hash::HashString,
    entry::{entry_type::AppEntryType, AppEntryValue, Entry},
    cas::content::Address,
    json::RawString,
};

use crate::stream::{
    Stream,
};

use crate::member::{
    handlers::get_my_member_id
};
use crate::utils;
use crate::message;
use crate::member;

pub fn handle_create_stream(
    name: String,
    description: String,
    initial_members: Vec<Address>,
    public: bool,
) -> ZomeApiResult<Address> {

    let stream = Stream{name, description, public};

    let entry = match public {
        true => Entry::App(
            AppEntryType::from("public_stream"),
            AppEntryValue::from(stream)
        ),
        false => Entry::App(
            AppEntryType::from("direct_stream"),
            AppEntryValue::from(stream)
        )
    };

    let stream_address = hdk::commit_entry(&entry)?;
    utils::link_entries_bidir(&get_my_member_id(), &stream_address, "member_of", "has_member")?;
    
    for member in initial_members {
        utils::link_entries_bidir(&member, &stream_address, "member_of", "has_member")?;
    }


    if public == true {
        let anchor_entry = Entry::App(
            AppEntryType::from("anchor"),
            AppEntryValue::from(RawString::from("public_streams")),
        );
        let anchor_address = hdk::commit_entry(&anchor_entry)?;
        hdk::link_entries(&anchor_address, &stream_address, "public_stream")?;
    }

    Ok(stream_address)
}

pub fn handle_add_members(stream_address: HashString, members: Vec<Address>) -> ZomeApiResult<()> {
    for member in members {
        utils::link_entries_bidir(&member, &stream_address, "member_of", "has_member")?;
    }
    Ok(())
}

pub fn handle_join_stream(stream_address: HashString) -> ZomeApiResult<()> {
    utils::link_entries_bidir(&member::handlers::get_my_member_id(), &stream_address, "member_of", "has_member")?;
    Ok(())
}

pub fn handle_get_my_streams() -> ZomeApiResult<utils::GetLinksLoadResult<Stream>> {
    utils::get_links_and_load_type(&get_my_member_id(), "member_of")
}


pub fn handle_get_members(address: HashString) -> ZomeApiResult<Vec<Address>> {
    let all_member_ids = hdk::get_links(&address, "has_member")?.addresses().to_owned();
    Ok(all_member_ids)
}

pub fn handle_get_messages(address: HashString) -> ZomeApiResult<utils::GetLinksLoadResult<message::Message>> {
    utils::get_links_and_load_type(&address, "message_in")
}

pub fn handle_post_message(stream_address: HashString, message_spec: message::MessageSpec) -> ZomeApiResult<()> {

    let message = message::Message::from_spec(
        &message_spec,
        &member::handlers::get_my_member_id().to_string());

    let message_entry = Entry::App(
        AppEntryType::from("message"),
        AppEntryValue::from(message)
    );

    let message_addr = hdk::commit_entry(&message_entry)?;

    hdk::link_entries(&stream_address, &message_addr, "message_in")?;

    Ok(())
}

pub fn handle_get_all_public_streams() -> ZomeApiResult<utils::GetLinksLoadResult<Stream>> {
    let anchor_entry = Entry::App(
        AppEntryType::from("anchor"),
        AppEntryValue::from(RawString::from("public_streams")),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    utils::get_links_and_load_type(&anchor_address, "public_stream")
}
