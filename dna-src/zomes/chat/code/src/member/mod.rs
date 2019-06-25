
use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
	error::JsonError,
    json::JsonString,
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

pub mod handlers;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Member {
    pub address: Address,
    pub profile: Profile
}

// This is the full profile that can be requested for a member
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Profile {
    pub name: String,
    pub avatar_url: String,
    pub address: Address,
}


pub fn profile_definition() -> ValidatingEntryType {
    entry!(
        name: "chat_profile",
        description: "The data that chat has about a particular user",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Profile>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: "profile",

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