extern crate utils;
use hdk::{
    self,
    AGENT_ADDRESS,
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        json::RawString,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};

use crate::stream::{
    Stream,
};

use utils::{
    GetLinksLoadResult,
    get_links_and_load_type
};
use crate::message;

fn notify_room(stream_address: Address, msg_type: String) -> ZomeApiResult<()> {
    let mut all_member_ids = hdk::get_links(&stream_address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    while let Some(member_id) = all_member_ids.pop() {
        if &AGENT_ADDRESS.to_string() == &member_id.to_string() {
            hdk::debug(format!("No need to send a message to myself: {:?}", &member_id.to_string())).ok();
        } else {
            hdk::debug(format!("Send a message to: {:?}", &member_id.to_string())).ok();
            hdk::send(member_id, json!({"msg_type": msg_type, "id": &stream_address}).to_string(), 10000.into())?;
        }
    }
    Ok(())
}

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

pub fn handle_join_stream(stream_address: Address) -> ZomeApiResult<()> {
    let mut all_member_ids = hdk::get_links(&stream_address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    let mut join_room = true;
    while let Some(member_id) = all_member_ids.pop() {
        if &AGENT_ADDRESS.to_string() == &member_id.to_string() {
            hdk::debug(format!("No need to rejoin roomMessages: {:?}", &member_id.to_string())).ok();
            join_room = false;
        }
    }
    if join_room {
        hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member", "", "")?;
        notify_room(stream_address, "new_room_member".to_string())?;
    }
    Ok(())
}

pub fn handle_get_members(address: Address) -> ZomeApiResult<Vec<Address>> {
    let all_member_ids = hdk::get_links(&address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    Ok(all_member_ids)
}

pub fn handle_get_messages(address: Address) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
    get_links_and_load_type(&address, LinkMatch::Exactly("message_in"), LinkMatch::Any)
}

pub fn handle_post_message(stream_address: Address, message_spec: message::MessageSpec) -> ZomeApiResult<()> {


    // Check DeepKey hash
    // Check Agent Key via bridge
    // Put validation into the message_entry

    let message = message::Message::from_spec(
        &message_spec,
        &AGENT_ADDRESS.to_string());

    let message_entry = Entry::App(
        "message".into(),
        message.into(),
    );

    let message_addr = hdk::commit_entry(&message_entry)?;

    hdk::link_entries(&stream_address, &message_addr, "message_in", "")?;

    notify_room(stream_address, "new_message".to_string())?;

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
