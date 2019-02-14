#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
	AGENT_ADDRESS,
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
	entry::Entry,
    hash::HashString,
    cas::content::Address,
    json::{RawString, JsonString},
    error::HolochainError,
};

mod anchor;
mod message;
mod stream;
mod member;
mod utils;

define_zome! {

	entries: [
		message::message_definition(),
    	stream::public_stream_definition(),
    	stream::direct_stream_definition(),
        member::profile_definition(),
        anchor::anchor_definition()
	]

    genesis: || {
        {
    		Ok(())
        }
    }

	functions: [
		register: {
			inputs: | |,
			outputs: |result: ZomeApiResult<Address>|,
			handler: handle_register
		}

		create_stream: {
			inputs: |name: String, description: String, initial_members: Vec<Address>, public: bool|,
			outputs: |result: ZomeApiResult<Address>|,
			handler: stream::handlers::handle_create_stream
		}
		get_my_streams: {
			inputs: | |,
			outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<stream::Stream>>|,
			handler: stream::handlers::handle_get_my_streams
		}
		get_all_public_streams: {
			inputs: | |,
			outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<stream::Stream>>|,
			handler: stream::handlers::handle_get_all_public_streams
		}
        get_all_members: {
			inputs: | |,
			outputs: |result: ZomeApiResult<Vec<Address>>|,
			handler: member::handlers::handle_get_all_members
		}
		get_members: {
			inputs: |stream_address: HashString|,
			outputs: |result: ZomeApiResult<Vec<Address>>|,
			handler: stream::handlers::handle_get_members
		}
		add_members: {
			inputs: |stream_address: HashString, members: Vec<Address>|,
			outputs: |result: ZomeApiResult<()>|,
			handler: stream::handlers::handle_add_members
		}
		join_stream: {
			inputs: |stream_address: HashString|,
			outputs: |result: ZomeApiResult<()>|,
			handler: stream::handlers::handle_join_stream
		}
		post_message: {
			inputs: |stream_address: HashString, message: message::MessageSpec|,
			outputs: |result: ZomeApiResult<()>|,
			handler: stream::handlers::handle_post_message
		}
		get_messages: {
			inputs: |address: HashString|,
			outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<message::Message>>|,
			handler: stream::handlers::handle_get_messages
		}
	]

	 traits: {
	        hc_public [register, create_stream, get_my_streams, get_all_public_streams, get_all_members, get_members, add_members, join_stream, post_message, get_messages, get_subjects, get_profile]
	}
 }

 fn handle_register() -> ZomeApiResult<Address> {
 	let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("member_directory").into(),
    );

    let anchor_address = hdk::commit_entry(&anchor_entry)?;

 	hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "member_tag")?;
 	Ok(AGENT_ADDRESS.to_string().into())
 }
