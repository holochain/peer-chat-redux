use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        // validation::EntryValidationData,
        // entry::Entry,
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
        validation: | _validation_data: hdk::EntryValidationData<Message>| {
            // match validation_data {
            //     EntryValidationData::Create{entry, validation_data} => {
            //     	let local_chain = validation_data.package.source_chain_entries
            //     		.ok_or("Could not retrieve source chain")?;
            //     	hdk::debug(format!("{:?}", local_chain))?;
            //
            //     	// load the game and game state
            //     	let _new_message = Message::from(entry);
            //
            //         // Sometimes the validating entry is already in the chain when validation runs,
            //         // To make our state reduction work correctly this must be removed
            //         // local_chain.remove_item(&Entry::App(MESSAGE_ENTRY.into() , _new_message.clone().into()));
            //
            //     	// let state = get_state_local_chain(local_chain.clone(), &_new_move.game)
            //     	// 	.map_err(|_| "Could not load state during validation")?;
            //     	// let game = get_game_local_chain(local_chain, &_new_move.game)
            //     	//     .map_err(|_| "Could not load game during validation")?;
            //         //
            //         // _new_move.is_valid(game, state)
            //         Ok(())
            //     },
            //     _ => {
            //         Err("Cannot modify or delete a move".into())
            //     }
            // }
            Ok(())
        }
    )
}
