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

    let conversation_address = hdk::commit_entry(&entry)?;
    hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &conversation_address, "member_of", "has_member", "", "")?;

    for member in initial_members {
        hdk::utils::link_entries_bidir(&member, &conversation_address, "member_of", "has_member", "", "")?;
    }

    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("public_conversations").into(),
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &conversation_address, "public_conversation", "")?;

    Ok(conversation_address)
}

pub fn handle_join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
    let mut all_member_ids = hdk::get_links(&conversation_address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned();
    let mut join_conversation = true;
    while let Some(member_id) = all_member_ids.pop() {
        if &AGENT_ADDRESS.to_string() == &member_id.to_string() {
            hdk::debug(format!("No need to rejoin conversationMessages: {:?}", &member_id.to_string())).ok();
            join_conversation = false;
        }
    }
    if join_conversation {
        hdk::utils::link_entries_bidir(&AGENT_ADDRESS, &conversation_address, "member_of", "has_member", "", "")?;
        notify_conversation(conversation_address, "new_conversation_member".to_string())?;
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
    let conversations: Vec<GetLinksLoadResult<Conversation>> = get_links_and_load_type(&anchor_address, LinkMatch::Exactly("public_conversation"), LinkMatch::Any)?;
    match conversations.len() {
        0 => {
            let conversation_address = create_ghost_conversation()?;
            let default_result = GetLinksLoadResult{
                entry: Conversation{name: "Ghost Chat".to_string(), description: "".to_string()},
                address: conversation_address.clone()
            };
            Ok(vec![default_result])
        },
        _ => {
            let result = conversations.iter().map(|elem| {
                GetLinksLoadResult {
                    entry: Conversation {
                        name: elem.entry.name.to_owned(),
                        description: elem.entry.description.to_owned()
                    },
                    address: elem.address.clone()
                }
            }).collect();
            Ok(result)
        }
    }
}

fn create_ghost_conversation() -> ZomeApiResult<Address> {
    hdk::debug("Create Default conversation")?;
    handle_start_conversation("Ghost Chat".to_string(), "".to_string(), [].to_vec())
}
