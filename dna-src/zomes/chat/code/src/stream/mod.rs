use hdk::{
    self,
    entry_definition::ValidatingEntryType,
};

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
	error::JsonError,
    json::JsonString,
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Stream {
    pub name: String,
    pub description: String
}


pub fn public_stream_definition() -> ValidatingEntryType {
    entry!(
        name: "public_stream",
        description: "A stream of which anyone can become a member and post",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Stream>| {
            Ok(())
        },

        links: [
            to!(
                "%agent_id",
                link_type: "has_member",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: "member_of",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "message",
                link_type: "message_in",

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
