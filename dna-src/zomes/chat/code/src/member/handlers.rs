use std::convert::TryInto;
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
use crate::profile_spec;
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
        profile_spec(),
    );
    hdk::debug(format!("{:?}", result)).unwrap();
    hdk::debug("register spec end")?;
    Ok(())
}

fn retrieve_profile(field_name: String) -> ZomeApiResult<String> {
    hdk::debug("retrieve_profile start")?;

    let result_json = hdk::call(
        "p-p-bridge",
        "profiles", 
        Address::from(PUBLIC_TOKEN.to_string()), // never mind this for now
        "retrieve",
        json!({"retriever_dna": Address::from(DNA_ADDRESS.to_string()), "profile_field": field_name}).into()
    )?;

    hdk::debug(format!("result of bridge call to retrieve: {:?}", result_json))?;

    result_json.try_into()?
}

pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
    hdk::utils::get_links_and_load_type(&agent_address, Some("profile".into()), None)?
        .iter()
        .next()
        .ok_or(ZomeApiError::Internal("Agent does not have a profile registered".into()))
        .map(|elem: &hdk::utils::GetLinksLoadResult<Profile>| {
            elem.entry.clone()
        })
}

pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
    match handle_get_member_profile(AGENT_ADDRESS.to_string().into()) {
        Ok(profile) => {
            // check if there any updates that have been made in P&P (if available)
            // fall back to using the existing saved info
            let name = retrieve_profile("handle".to_string()).unwrap_or(profile.name);
            let avatar_url = retrieve_profile("avatar".to_string()).unwrap_or(profile.avatar_url);
            Ok(Profile {
                name,
                avatar_url,
                ..profile
            })
        },
        Err(_) => {
            match (retrieve_profile("handle".to_string()), retrieve_profile("avatar".to_string())) {
                (Ok(handle), Ok(avatar)) => {
                    // handle and avatar both successfully retrieved from P&P
                    // register them then return the profile
                    handle_register(handle.clone(), avatar.clone())?;
                    hdk::debug("Profile details registered").ok();
                    Ok(Profile {
                        name: handle,
                        avatar_url: avatar,
                        address: AGENT_ADDRESS.to_string().into(),
                    })
                }
                _ => {
                    // no handle or avatar in P&P
                    // register the spec then trigger redirect
                    register_spec().unwrap();
                    hdk::debug("Spec registered").ok();
                    Err(ZomeApiError::Internal(DNA_ADDRESS.to_string()))
                }
            }
        },
    }
}
