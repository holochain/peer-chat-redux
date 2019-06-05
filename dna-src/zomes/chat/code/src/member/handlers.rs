use std::convert::TryInto;
use hdk::{
    PUBLIC_TOKEN,
    AGENT_ADDRESS,
    DNA_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        json::{RawString},
        json::{JsonString},
        cas::content::Address,
    },
    error::{
        ZomeApiResult,
        ZomeApiError,
    }
};

use crate::member::Profile;
use hdk::utils::GetLinksLoadResult;

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
          "sourceDna": DNA_ADDRESS.to_string(),
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
                }]}}).into()
    );
    hdk::debug(format!("{:?}", result)).unwrap();
    hdk::debug("register spec end")?;
    Ok(())
}

fn retrieve_profile(field_name: String) -> ZomeApiResult<JsonString> {
    hdk::debug("retrieve_profile start")?;
    let result_json = hdk::call("p-p-bridge", "profiles", Address::from(PUBLIC_TOKEN.to_string()), // never mind this for now
        "retrieve",
        json!({"retriever_dna": Address::from(DNA_ADDRESS.to_string()), "profile_field": field_name}).into()
    );
    hdk::debug(format!("{:?}", &result_json))?;
    hdk::debug(format!("********DEBUG******** BRIDGING ACTUAL response from retrieve_field {:?}", &result_json)).ok();
    hdk::debug("retrieve_profile end")?;
    result_json
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    hdk::utils::get_links_and_load_type(&agent_address, Some("profile".into()), None)?
        .into_iter()
        .next()
        .ok_or_else(|| {
            let maybe_handle = retrieve_profile("handle".to_string());
            let maybe_avatar = retrieve_profile("avatar".to_string());
            hdk::debug(format!("saved handle {:?}", maybe_handle)).ok();
            hdk::debug(format!("saved avatar {:?}", maybe_avatar)).ok();
            match (maybe_handle, maybe_avatar) {
                (Ok(handle), Ok(avatar)) => {
                    let inner_handle: ZomeApiResult<String> = handle.clone().try_into().unwrap();
                    let inner_avatar: ZomeApiResult<String> = avatar.clone().try_into().unwrap();
                    match (inner_handle, inner_avatar) {
                        (Ok(saved_handle), Ok(saved_avatar)) => {
                            hdk::debug(format!("saved_handle {:?}", saved_handle)).ok();
                            hdk::debug(format!("saved_avatar {:?}", saved_avatar)).ok();
                            handle_register(saved_handle.try_into().unwrap(), saved_avatar.try_into().unwrap()).unwrap();
                            hdk::debug("Profile details registered").ok();
                            ZomeApiError::Internal(DNA_ADDRESS.to_string())
                        }
                        _ => {
                            register_spec().unwrap();
                            hdk::debug("Spec registered").ok();
                            ZomeApiError::Internal(DNA_ADDRESS.to_string())
                        }
                    }
                }
                _ => unreachable!()
            }
        })
        .map(|elem: GetLinksLoadResult<Profile>| {
            elem.entry.clone()
        })
}

pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
    handle_get_member_profile(AGENT_ADDRESS.to_string().into())
}
