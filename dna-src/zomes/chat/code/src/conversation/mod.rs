use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
    	error::JsonError,
        json::JsonString,
    },
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Conversation {
    pub name: String,
    pub description: String
}

use crate::{
    PUBLIC_STREAM_ENTRY,
    PUBLIC_STREAM_LINK_TYPE_TO,
    PUBLIC_STREAM_LINK_TYPE_FROM,
    MESSAGE_LINK_TYPE_TO
};

pub fn public_conversation_definition() -> ValidatingEntryType {
    entry!(
        name: PUBLIC_STREAM_ENTRY,
        description: "A conversation of which anyone can become a member and post",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Conversation>| {
            Ok(())
        },

        links: [
            to!(
                "%agent_id",
                link_type: PUBLIC_STREAM_LINK_TYPE_TO,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: PUBLIC_STREAM_LINK_TYPE_FROM,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "message",
                link_type: MESSAGE_LINK_TYPE_TO,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
