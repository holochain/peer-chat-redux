module.exports = scenario => {

  scenario('Agent 2 requests full_name from Agent 1', async (s, t, {chat_instance_1, personas_instance_1, chat_instance_2}) => {
    const get_profile_result_1 = await chat_instance_1.call('chat', 'get_my_member_profile', {})
    let sourceDna = get_profile_result_1.Err.Internal
    console.log('sourceDna ' + JSON.stringify(sourceDna))

    const get_result = await personas_instance_1.call('profiles', 'get_profiles', {})
    // console.log('Profiles' + JSON.stringify(get_result))
    t.deepEqual(get_result.Ok.length, 2)

    // create a persona to map to and add a field
    const result = await personas_instance_1.callSync("personas", "create_persona", {spec: {name: "test"}})
    const persona_address = result.Ok
    const field_handle = await personas_instance_1.callSync("personas", "add_field", {persona_address: persona_address, field: {name: "handle", data: "@philt3r"}})
    console.log(field_handle)
    await personas_instance_1.callSync("personas", "add_field", {persona_address: persona_address, field: {name: "avatar", data: "avatar-data"}})
    await personas_instance_1.callSync("personas", "add_field", {persona_address: persona_address, field: {name: "full_name", data: "Philip"}})
    await personas_instance_1.callSync("personas", "add_field", {persona_address: persona_address, field: {name: "last_name", data: "Beadle"}})

    // can callSync the function to create a mapping
    await personas_instance_1.callSync("profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "handle",
          personaAddress: persona_address,
          personaFieldName: "handle"
        }
    })
    await personas_instance_1.callSync("profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "avatar",
          personaAddress: persona_address,
          personaFieldName: "avatar"
        }
    })
    await personas_instance_1.callSync("profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "full_name",
          personaAddress: persona_address,
          personaFieldName: "full_name"
        }
    })
    await personas_instance_1.callSync("profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "last_name",
          personaAddress: persona_address,
          personaFieldName: "last_name"
        }
    })

    const get_profiles = await personas_instance_1.callSync("profiles", "get_profiles", {})
    console.log(get_profiles.Ok[1])
    t.deepEqual(get_profiles.Ok.filter(p => p.name === "Holochain Peer Chat")[0].fields[0].mapping, {personaAddress: persona_address, personaFieldName: 'handle'})

    const get_personas = await personas_instance_1.callSync("personas", "get_personas", {})
    console.log(get_personas.Ok)

    const get_persona_field = await personas_instance_1.callSync("personas", "get_field", {persona_address: persona_address, field_name: "full_name"})
    console.log(get_persona_field)

    const get_full_name_received = await chat_instance_2.call('chat', 'get_full_name', {agent_address: chat_instance_1.agentId})
    console.log('get_full_name_received', get_full_name_received.Ok)

    t.deepEqual(get_full_name_received.Ok.body, 'Philip')

  })
}
