#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
#[macro_use]
extern crate holochain_json_derive;
extern crate utils;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate validator;
use std::convert::TryInto;
use hdk::{
	api::DNA_ADDRESS,
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
	holochain_core_types::{
		validation::EntryValidationData,
		agent::AgentId,
	},
	holochain_persistence_api::{
		cas::content::Address,
	},
	holochain_json_api::{
		json::JsonString,
		error::JsonError,
	},
};

use hdk_proc_macros::zome;
use utils::GetLinksLoadResult;

pub mod anchor;
pub mod message;
pub mod conversation;
pub mod member;

pub static DEEPKEY_ADDRESS: &str = "QmDeepKeyHash";
pub static MESSAGE_ENTRY: &str = "message";
pub static MESSAGE_LINK_TYPE_TO: &str = "message_in";
pub static PUBLIC_STREAM_ENTRY: &str = "public_conversation";
pub static PUBLIC_STREAM_LINK_TYPE_TO: &str = "has_member";
pub static PUBLIC_STREAM_LINK_TYPE_FROM: &str = "member_of";

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
struct Message {
	msg_type: String,
	id: String,
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct SignalPayload {
	conversation_id: String
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct NamePayload {
	name: String
}



#[zome]
pub mod chat {

	#[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
		if let EntryValidationData::Create{entry, ..} = validation_data {
			let properties = hdk::api::property("allowed_members");
			if let Ok(members) = properties {
				let member_list: Vec<String> = serde_json::from_str(&members.to_string()).unwrap();
				hdk::debug(format!("PROPERTIES: {:?}", member_list)).ok();
				let agent = entry as AgentId;
				hdk::debug(format!("AgentId: {:?}", agent)).ok();
				if member_list.contains(&agent.pub_sign_key) {
					Ok(())
				} else if member_list.contains(&"public".to_string()) {
					Ok(())
				} else {
					Err("This agent is not in the allowed members list".into())
				}
			} else {
				Err("Issue reading members from dna.json".into())
			}
	    } else {
			hdk::debug(format!("Cannot update or delete an agent at this time")).ok();
	        Err("Cannot update or delete an agent at this time".into())
	    }
    }

	#[receive]
	pub fn receive(from: Address, msg_json: JsonString) -> String {
		hdk::debug(format!("New message from: {:?}", from)).ok();
		let maybe_message: Result<Message, _> = JsonString::from_json(&msg_json).try_into();
		match maybe_message {
			Err(err) => format!("error: {}", err),
			Ok(message) => match message.msg_type.as_str() {
				"new_conversation_member" | "new_message" => {
					let conversation_id = message.id;
					let _ = hdk::emit_signal(message.msg_type.as_str(), SignalPayload{conversation_id});
					json!({
						"msg_type": message.msg_type.as_str(),
						"body": format!("Emit: {}", message.msg_type.as_str())
					})
					.to_string()
				}
				"full_name_request" => {
					let name = member::handlers::retrieve_profile("full_name".into()).expect("full_name_request Couldn't find full_name");
					json!({
						"msg_type": message.msg_type.as_str(),
						"body": format!("{}", name)
					})
					.to_string()
				}
				_ => {
					json!({
						"msg_type": message.msg_type.as_str(),
						"body": format!("No match: {}", message.msg_type.as_str())
					})
					.to_string()
				}
			}
		}
	}

	#[entry_def]
    pub fn message_entry_def() -> ValidatingEntryType {
        message::message_definition()
    }

	#[entry_def]
    pub fn public_conversation_entry_def() -> ValidatingEntryType {
        conversation::public_conversation_definition()
    }

	#[entry_def]
    pub fn member_entry_def() -> ValidatingEntryType {
		member::profile_definition()
    }

	#[entry_def]
    pub fn allowed_members_entry_def() -> ValidatingEntryType {
		member::allowed_members_definition()
    }

	#[entry_def]
	pub fn anchor_entry_def() -> ValidatingEntryType {
		anchor::anchor_definition()
	}

	#[zome_fn("hc_public")]
    pub fn register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
        member::handlers::handle_register(name, avatar_url)
    }

	#[zome_fn("hc_public")]
    pub fn start_conversation(name: String, description: String, initial_members: Vec<Address>) -> ZomeApiResult<Address> {
        conversation::handlers::handle_start_conversation(name, description, initial_members)
    }

	#[zome_fn("hc_public")]
	pub fn join_conversation(conversation_address: Address) -> ZomeApiResult<()> {
		conversation::handlers::handle_join_conversation(conversation_address)
	}

	#[zome_fn("hc_public")]
	pub fn get_all_public_conversations() -> ZomeApiResult<Vec<GetLinksLoadResult<conversation::Conversation>>> {
		conversation::handlers::handle_get_all_public_conversations()
	}

	#[zome_fn("hc_public")]
	pub fn get_members(conversation_address: Address) -> ZomeApiResult<Vec<Address>> {
		conversation::handlers::handle_get_members(conversation_address)
	}

	#[zome_fn("hc_public")]
	pub fn get_member_profile(agent_address: Address) -> ZomeApiResult<member::Profile> {
		member::handlers::handle_get_member_profile(agent_address)
	}

	#[zome_fn("hc_public")]
	pub fn get_my_member_profile() -> ZomeApiResult<member::Profile> {
		member::handlers::handle_get_my_member_profile()
	}

	#[zome_fn("hc_public")]
	pub fn get_full_name(agent_address: Address) -> ZomeApiResult<JsonString> {
		member::handlers::handle_get_full_name(agent_address)
	}

	#[zome_fn("hc_public")]
	pub fn post_message(conversation_address: Address, message: message::MessageSpec) -> ZomeApiResult<()> {
		conversation::handlers::handle_post_message(conversation_address, message)
	}

	#[zome_fn("hc_public")]
	pub fn get_messages(address: Address) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
		conversation::handlers::handle_get_messages(address)
	}
 }

pub fn profile_spec() -> JsonString{
	json!(
	{
		"spec": {
		  "name": "Holochain Peer Chat",
		  "sourceDna": DNA_ADDRESS.to_string(),
		  "fields": [
		  		{
		            "name": "handle",
		            "displayName": "Handle",
		            "required": true,
		            "description": "This is the name other people you chat to will see. ",
		            "usage": "STORE",
		            "schema": ""
		        },
		        {
		            "name": "avatar",
		            "displayName": "Avatar",
		            "required": true,
		            "description": "",
		            "usage": "STORE",
		            "schema": ""
		        },
		        {
		            "name": "full_name",
		            "displayName": "Full Name",
		            "required": false,
		            "description": "Your name will show when someone clicks it in the members list if you are online",
		            "usage": "DISPLAY",
		            "schema": ""
		        }
		    ]
	    }
	}
	).into()
}
