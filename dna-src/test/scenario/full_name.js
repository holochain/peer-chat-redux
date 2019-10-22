module.exports = scenario => {

  const { config1, config2 } = require('../config')

  scenario.only('Agent 2 requests full_name from Agent 1', async (s, t) => {
    const {player1, player2} = await s.players({player1: config1, player2: config2}, true)
    const get_profile_result_1 = await player1.call('chat', 'chat', 'get_my_member_profile', {})
    let sourceDna = get_profile_result_1.Err.Internal
    console.log('sourceDna ' + JSON.stringify(sourceDna))

    const get_result = await player1.call("personas", 'profiles', 'get_profiles', {})
    console.log('Profiles' + JSON.stringify(get_result))
    t.deepEqual(get_result.Ok.length, 2)

    // create a persona to map to and add a field
    const result = await player1.call("personas", "personas", "create_persona", {spec: {name: "test"}})
    const persona_address = result.Ok
    const field_handle = await player1.call("personas", "personas", "add_field", {persona_address: persona_address, field: {name: "handle", data: "@philt3r"}})
    console.log(field_handle)
    await player1.call("personas", "personas", "add_field", {persona_address: persona_address, field: {name: "avatar", data: "avatar-data"}})
    await player1.call("personas", "personas", "add_field", {persona_address: persona_address, field: {name: "full_name", data: "Philip"}})
    await player1.call("personas", "personas", "add_field", {persona_address: persona_address, field: {name: "last_name", data: "Beadle"}})
    await s.consistency()

    // can call the function to create a mapping
    await player1.call("personas", "profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "handle",
          personaAddress: persona_address,
          personaFieldName: "handle"
        }
    })
    await player1.call("personas", "profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "avatar",
          personaAddress: persona_address,
          personaFieldName: "avatar"
        }
    })
    await player1.call("personas", "profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "full_name",
          personaAddress: persona_address,
          personaFieldName: "full_name"
        }
    })
    await player1.call("personas", "profiles", "create_mapping",
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: "last_name",
          personaAddress: persona_address,
          personaFieldName: "last_name"
        }
    })
    await s.consistency()

    const get_profiles = await player1.call("personas", "profiles", "get_profiles", {})
    console.log(get_profiles.Ok[1])
    t.deepEqual(get_profiles.Ok.filter(p => p.name === "Holochain Peer Chat")[0].fields[0].mapping, {personaAddress: persona_address, personaFieldName: 'handle'})

    const get_personas = await player1.call("personas", "personas", "get_personas", {})
    console.log(get_personas.Ok)

    const get_persona_field = await player1.call("personas", "personas", "get_field", {persona_address: persona_address, field_name: "full_name"})
    await s.consistency()
    console.log(get_persona_field)

    const get_full_name_received = await player2.call('chat', 'chat', 'get_full_name', {agent_address: player1.info('chat').agentAddress})
    await s.consistency()

    console.log('get_full_name_received', get_full_name_received)

    t.deepEqual(get_full_name_received.Ok.body, 'Philip')

  })
}
