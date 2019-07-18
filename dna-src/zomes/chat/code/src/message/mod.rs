use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Message {
    pub timestamp: u32,
    pub author: String,
    pub message_type: String,
    pub payload: String,
    pub meta: String,
}

impl Message {
    pub fn from_spec(spec: &MessageSpec, author: &String) -> Message {
        return Message{
            message_type: spec.message_type.clone(),
            payload: spec.payload.clone(),
            meta: spec.meta.clone(),
            author: author.to_owned(),
            timestamp: spec.timestamp.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct MessageSpec {
    pub message_type: String,
    pub timestamp: u32,
    pub payload: String,
    pub meta: String
}

use crate::{
    MESSAGE_ENTRY,
};

pub fn message_definition() -> ValidatingEntryType {
    entry!(
        name: MESSAGE_ENTRY,
        description: "A generic message entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Message>| {
            Ok(())
        }
    )
}
