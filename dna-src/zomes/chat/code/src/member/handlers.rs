
use hdk::{
    PUBLIC_TOKEN,
    AGENT_ADDRESS,
    DNA_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        json::{RawString},
        cas::content::Address,

    },
    error::{
        ZomeApiResult,
        ZomeApiError,
    }
};

use crate::member::Profile;
use crate::utils;

use serde_json::json;

pub fn handle_register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("member_directory").into(),
    );

    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "member_tag", "")?;

    let profile_entry = Entry::App(
        "chat_profile".into(),
        Profile {
        name,
        avatar_url,
        address: AGENT_ADDRESS.to_string().into()
    }.into()
    );
    let profile_addr = hdk::commit_entry(&profile_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &profile_addr, "profile", "")?;

    Ok(AGENT_ADDRESS.to_string().into())
}

fn register_spec() -> ZomeApiResult<()> {
    hdk::debug("register spec start")?;
    let result = hdk::call("p-p-bridge", "profiles", Address::from(PUBLIC_TOKEN.to_string()), // never mind this for now
        "register_app",
        json!({"spec": {
          "name": "holochain-basic-chat",
          "sourceDna": Address::from(DNA_ADDRESS.to_string()),
          "fields": [{
                    "name": "handle",
                    "displayName": "Handle",
                    "required": true,
                    "description": "This is the name other people you cha to will see. ",
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
                    "required": true,
                    "description": "Your name will show when someone clicks it in the members list if you are online",
                    "usage": "DISPLAY",
                    "schema": ""
                }]}}).into()
    );
    hdk::debug(format!("{:?}", result)).unwrap();
    hdk::debug("register spec end")?;
    Ok(())
}

fn retrieve_profile(field_name: String) -> ZomeApiResult<String> {
    hdk::debug("retrieve_profile start")?;
    let result = hdk::call("p-p-bridge", "profiles", Address::from(PUBLIC_TOKEN.to_string()), // never mind this for now
        "retrieve",
        json!({"retriever_dna": Address::from(DNA_ADDRESS.to_string(), "profile_field": field_name}).into()
    );
    hdk::debug(format!("{:?}", result)).unwrap();
    hdk::debug("retrieve_profile end")?;
    match
    Err(())
    Ok((result))
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    utils::get_links_and_load_type(&agent_address, "profile")?
        .iter()
        .next()
        .ok_or_else(|| {
            let handle = retrieve_profile("handle".to_string()).unwrap();
            if handle is an err {
                register_spec().unwrap();
                ZomeApiError::Internal(Address::from(DNA_ADDRESS.to_string())
            } else {
                // register the profile
                let avatar = retrieve_profile("avatar".to_string()).unwrap();
                handle_register(handle, avatar);
                ZomeApiError::Internal(Address::from(DNA_ADDRESS.to_string())
            }
        })
        .map(|elem: &utils::GetLinksLoadElement<Profile>| {
            elem.entry.clone()
        })
}

pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
    handle_get_member_profile(AGENT_ADDRESS.to_string().into())
}
