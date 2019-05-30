use hdk::entry_definition::ValidatingEntryType;

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    json::RawString,
};


pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<RawString>| {
            Ok(())
        },

        links: [
            to!(
                "%agent_id",
                link_type: "member_tag",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "public_stream",
                link_type: "public_stream",

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