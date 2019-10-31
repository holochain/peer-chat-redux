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
        cas::content::{Address, AddressableContent},
    },
};
use std::collections::HashSet;

use crate::conversation::{
    Conversation,
};

use utils::{
    GetLinksLoadResult,
    get_links_and_load_type
};
use crate::message;

fn notify_conversation(conversation_address: Address, msg_type: String) -> ZomeApiResult<()> {
    let mut all_member_ids = hdk::get_links(&conversation_address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    while let Some(member_id) = all_member_ids.pop() {
        if &AGENT_ADDRESS.to_string() == &member_id.to_string() {
            hdk::debug(format!("No need to send a message to myself: {:?}", &member_id.to_string())).ok();
        } else {
            hdk::debug(format!("Send a message to: {:?}", &member_id.to_string())).ok();
            hdk::send(member_id, json!({"msg_type": msg_type, "id": &conversation_address}).to_string(), 10000.into())?;
        }
    }
    Ok(())
}

pub fn handle_start_conversation(
    name: String,
    description: String,
    initial_members: Vec<Address>,
) -> ZomeApiResult<Address> {
    let conversation = Conversation{name, description};
    let entry = Entry::App(
        "public_conversation".into(),
        conversation.into()
    );

    // first see if this converstion already exists.
    // while the entry will de-dup, the links will not adding useless data to the DHT.
    // There is also the possibility this chat has already been created but we just don't see it.
    // In this case there is nothing we can do except add it again and de-dup
    // in the get_channels zome function
    let conversation_address = entry.address();
    if hdk::get_entry(&conversation_address)?.is_none() {
        hdk::commit_entry(&entry)?;
        let anchor_entry = Entry::App(
            "anchor".into(),
            RawString::from("public_conversations").into(),
        );
        let anchor_address = hdk::commit_entry(&anchor_entry)?;
        hdk::link_entries(&anchor_address, &conversation_address, "public_conversation", "")?;
    }
    let existing_members = handle_get_members(conversation_address.clone())?;

    // add the new members (including the creator)
    let mut members_to_add = vec![Address::from(AGENT_ADDRESS.to_string())];
    members_to_add.extend(initial_members);
    for member in members_to_add {
        if !existing_members.contains(&member) {
            hdk::utils::link_entries_bidir(&member, &conversation_address, "member_of", "has_member", "", "")?;
        }
    }
    Ok(conversation_address)
}

pub fn handle_join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
    let existing_members = handle_get_members(conversation_address.clone())?;
    if !existing_members.contains(&AGENT_ADDRESS) {
        hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &conversation_address, "member_of", "has_member", "", "")?;
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

pub fn handle_post_message(conversation_address: Address, message_spec: message::MessageSpec) -> ZomeApiResult<()> {
    let message = message::Message::from_spec(
        &message_spec,
        &AGENT_ADDRESS.to_string());

    let message_entry = Entry::App(
        "message".into(),
        message.into(),
    );

    let message_addr = hdk::commit_entry(&message_entry)?;

    hdk::link_entries(&conversation_address, &message_addr, "message_in", "")?;

    notify_conversation(conversation_address, "new_message".to_string())?;

    Ok(())
}

pub fn handle_get_all_public_conversations() -> ZomeApiResult<Vec<GetLinksLoadResult<Conversation>>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_conversations").into(),
    );
    let anchor_address = hdk::entry_address(&anchor_entry)?;
    let mut result = get_links_and_load_type(&anchor_address, LinkMatch::Exactly("public_conversation"), LinkMatch::Any)?;
    // dedup any channels that managed to slip through the dedup on write check
    // perhaps because we couldn't see them at the time of creation
    let mut uniques = HashSet::new();
    result.retain(|e| uniques.insert(e.address.clone()));
    Ok(result)
}
