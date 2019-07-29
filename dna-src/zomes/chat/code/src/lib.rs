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
use std::convert::TryInto;
use hdk::{
	api::DNA_ADDRESS,
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
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
pub mod stream;
pub mod member;

pub static MESSAGE_ENTRY: &str = "message";
pub static MESSAGE_LINK_TYPE_TO: &str = "message_in";
pub static PUBLIC_STREAM_ENTRY: &str = "public_stream";
pub static PUBLIC_STREAM_LINK_TYPE_TO: &str = "has_member";
pub static PUBLIC_STREAM_LINK_TYPE_FROM: &str = "member_of";

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
struct Message {
	msg_type: String,
	room_id: String,
}

#[derive(Debug, Serialize, Deserialize, DefaultJson)]
#[serde(rename_all = "camelCase")]
struct SignalPayload {
	room_id: String
}

#[zome]
pub mod chat {

    #[genesis]
    fn genesis() {
        Ok(())
    }

	#[receive]
	pub fn receive(from: Address, msg_json: JsonString) {
		hdk::debug(format!("New message from: {:?}", from)).ok();
		let maybe_message: Result<Message, _> = JsonString::from_json(&msg_json).try_into();
		match maybe_message {
			Err(err) => format!("error: {}", err),
			Ok(message) => match message.msg_type.as_str() {
				"new_room_member" | "new_message" => {
					let room_id = message.room_id;
					let _ = hdk::emit_signal(message.msg_type.as_str(), SignalPayload{room_id});
					format!("Emitted: {}", message.msg_type.as_str())
				}
				"full_name_request" => {
					format!("Emitted: {}", message.msg_type.as_str())
				}
				_ => {
					format!("No match: {}", message.msg_type.as_str())
				}
			}
		}
	}

	#[entry_def]
    pub fn message_entry_def() -> ValidatingEntryType {
        message::message_definition()
    }

	#[entry_def]
    pub fn public_stream_entry_def() -> ValidatingEntryType {
        stream::public_stream_definition()
    }

	#[entry_def]
    pub fn member_entry_def() -> ValidatingEntryType {
		member::profile_definition()
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
    pub fn create_stream(name: String, description: String, initial_members: Vec<Address>) -> ZomeApiResult<Address> {
        stream::handlers::handle_create_stream(name, description, initial_members)
    }

	#[zome_fn("hc_public")]
	pub fn join_stream(stream_address: Address) -> ZomeApiResult<()> {
		stream::handlers::handle_join_stream(stream_address)
	}

	#[zome_fn("hc_public")]
	pub fn get_all_public_streams() -> ZomeApiResult<Vec<GetLinksLoadResult<stream::Stream>>> {
		stream::handlers::handle_get_all_public_streams()
	}

	#[zome_fn("hc_public")]
	pub fn get_members(stream_address: Address) -> ZomeApiResult<Vec<Address>> {
		stream::handlers::handle_get_members(stream_address)
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
	pub fn post_message(stream_address: Address, message: message::MessageSpec) -> ZomeApiResult<()> {
		stream::handlers::handle_post_message(stream_address, message)
	}

	#[zome_fn("hc_public")]
	pub fn get_messages(address: Address) -> ZomeApiResult<Vec<GetLinksLoadResult<message::Message>>> {
		stream::handlers::handle_get_messages(address)
	}
 }

pub fn profile_spec() -> JsonString{
	json!(
	{
		"spec": {
		  "name": "holochain-basic-chat",
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
		            "name": "first_name",
		            "displayName": "First Name",
		            "required": false,
		            "description": "Your name will show when someone clicks it in the members list if you are online",
		            "usage": "DISPLAY",
		            "schema": ""
		        },
		        {
		            "name": "last_name",
		            "displayName": "Last Name",
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
