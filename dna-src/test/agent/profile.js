module.exports = scenario => {

  // scenario('Register the Profile Spec when there is no existing profile', async (s, t, {chat_instance_1, personas_instance_1}) => {
  //   const get_profile_result_1 = await chat_instance_1.call('chat', 'get_my_member_profile', {})
  //   let sourceDna = get_profile_result_1
  //   console.log('sourceDna ' + JSON.stringify(sourceDna))
  //   const get_result = await personas_instance_1.call('profiles', 'get_profiles', {})
  //   console.log('Profiles' + JSON.stringify(get_result))
  //   t.deepEqual(get_result.Ok.length, 2)
  // })

  scenario('Register a Profile Spec and map handle field to persona', async (s, t, {chat_instance_1, personas_instance_1, chat_instance_2}) => {
    const get_profile_result_1 = await chat_instance_1.call('chat', 'get_my_member_profile', {})
    let sourceDna = get_profile_result_1.Err.Internal
    console.log('sourceDna ' + JSON.stringify(sourceDna))

    const get_result = await personas_instance_1.call('profiles', 'get_profiles', {})
    console.log('Profiles' + JSON.stringify(get_result))
    t.deepEqual(get_result.Ok.length, 2)

    // create a persona to map to and add a field
    const result = await personas_instance_1.callSync("personas", "create_persona", {spec: {name: "test"}})
    const persona_address = result.Ok
    const add_result = await personas_instance_1.callSync("personas", "add_field", {personaAddress: persona_address, field: {name: "handle", data: "@philt3r"}})

    // can callSync the function to create a mapping
    const map_result2 = await personas_instance_1.callSync("profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "handle",
          personaAddress: persona_address,
          personaFieldName: "handle"
        }
      })
    console.log(map_result2)

    // should map a single field
    t.deepEqual(map_result2.Ok, { mappingsCreated: 1 }, "a single mapping should be created");

    // can then see the field is mapped
    const get_profiles = await personas_instance_1.callSync("profiles", "get_profiles", {})
    console.log(get_profiles)
    t.deepEqual(get_profiles.Ok.filter(p => p.name === "holochain-basic-chat")[0].fields[0].mapping, {personaAddress: persona_address, personaFieldName: 'handle'})
  })

  scenario('Can register a profile and retrieve', async (s, t, {chat_instance_1}) => {
    const register_result = await chat_instance_1.call('chat', 'register', {name: 'alice', avatar_url: ''})
    console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const get_profile_result = await chat_instance_1.call('chat', 'get_member_profile', {agent_address: register_result.Ok})
    console.log(get_profile_result)
  })
}
