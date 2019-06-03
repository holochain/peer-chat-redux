use std::convert::TryInto;
use hdk::{
    PUBLIC_TOKEN,
    AGENT_ADDRESS,
    DNA_ADDRESS,
    holochain_core_types::{
        entry::Entry,
        json::{RawString},
        cas::content::Address,
        error::HolochainError,
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

fn retrieve_profile(field_name: String) -> ZomeApiResult<RawString> {
    hdk::debug("retrieve_profile start")?;
    let result_json = hdk::call("p-p-bridge", "profiles", Address::from(PUBLIC_TOKEN.to_string()), // never mind this for now
        "retrieve",
        json!({"retriever_dna": Address::from(DNA_ADDRESS.to_string()), "profile_field": field_name}).into()
    )?;
    hdk::debug(format!("{:?}", &result_json))?;

    let entry: Result<RawString, HolochainError> = result_json.try_into();

    // let result: ZomeApiResult<RawString> = result_json.try_into()?;
    // let result_serde_json: serde_json::Value = serde_json::from_str(result_json.to_string().as_ref()).unwrap();
    // let entry: RawString = result_serde_json["Ok"].as_str().unwrap().into();

    // let entry = result_json.try_into()?;
    hdk::debug(format!("********DEBUG******** BRIDGING ACTUAL response from retrieve_field {:?}", entry))?;
    hdk::debug("retrieve_profile end")?;
    Ok(entry.unwrap())
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    utils::get_links_and_load_type(&agent_address, "profile")?
        .iter()
        .next()
        .ok_or_else(|| {
            let maybe_handle = retrieve_profile("handle".to_string());
            let maybe_avatar = retrieve_profile("avatar".to_string());
            // hdk::debug(format!("handle {:?}", handle));
            match (maybe_handle, maybe_avatar) {
                (Ok(handle), Ok(avatar)) => {
                    handle_register(handle.into(), avatar.into()).unwrap();
                    ZomeApiError::Internal(DNA_ADDRESS.to_string())
                }
                _ => {
                    register_spec().unwrap();
                    ZomeApiError::Internal(DNA_ADDRESS.to_string())
                }
            }
        })
        .map(|elem: &utils::GetLinksLoadElement<Profile>| {
            elem.entry.clone()
        })
}

pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
    handle_get_member_profile(AGENT_ADDRESS.to_string().into())
}
