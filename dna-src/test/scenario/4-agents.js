module.exports = scenario => {

  const { config1, config2, config3, config4 } = require('../config')

  const convoHoloscape = {
    name: 'Holoscape rox!',
    description: '',
    initial_members: [],
    public: true
  }

  const convoHoloscapeMessage1 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'This is Philip chatting in the Holoscape conversation...',
    meta: '{}',
  }

  const convoHoloscapeMessage2 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'This is Willem chatting in the Holoscape conversation...',
    meta: '{}',
  }

  const convoHoloscapeMessage3 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'Phil posting again in the Holoscape conversation...',
    meta: '{}',
  }

  const convoHoloscapeMessage4 = {
    timestamp: 0,
    message_type: 'text',
    payload: 'Art posting in the Holoscape conversation...',
    meta: '{}',
  }

  async function create_persona_profile(s, t, player, handle, avatar, fullName) {
    console.log('create_persona_profile')
    const result_get_my_member_profile = await player.call('chat', 'chat', 'get_my_member_profile', {})
    const sourceDna = result_get_my_member_profile.Err.Internal
    console.log('sourceDna ' + JSON.stringify(sourceDna))
    const result = await player.call('personas', 'personas', 'create_persona', {spec: {name: 'Personal'}})
    const persona_address = result.Ok
    const field_handle = await player.call('personas', 'personas', 'add_field', {persona_address: persona_address, field: {name: 'handle', data: handle}})
    console.log(field_handle)
    await player.call('personas', 'personas', 'add_field', {persona_address: persona_address, field: {name: 'avatar', data: avatar}})
    await player.call('personas', 'personas', 'add_field', {persona_address: persona_address, field: {name: 'full_name', data: fullName}})
    await s.consistency()

    // can call the function to create a mapping
    await player.call('personas', 'profiles', 'create_mapping',
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: 'handle',
          personaAddress: persona_address,
          personaFieldName: 'handle'
        }
    })
    await player.call('personas', 'profiles', 'create_mapping',
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: 'avatar',
          personaAddress: persona_address,
          personaFieldName: 'avatar'
        }
    })
    await player.call('personas', 'profiles', 'create_mapping',
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: 'full_name',
          personaAddress: persona_address,
          personaFieldName: 'full_name'
        }
    })
    await player.call('personas', 'profiles', 'create_mapping',
      {
        mapping: {
          retrieverDna: sourceDna,
          profileFieldName: 'last_name',
          personaAddress: persona_address,
          personaFieldName: 'last_name'
        }
    })
    await s.consistency()
    const my_member_profile = await player.call('chat', 'chat', 'get_my_member_profile', {})
    await s.consistency()
    return my_member_profile
  }

  scenario('4 Agents chatting', async (s, t) => {
    const {player1} = await s.players({player1: config1}, false)
    await player1.spawn()
    // await player1.kill()
    const my_member_profile = await create_persona_profile(s, t, player1, '@philt3r', 'avatar', 'Philip Beadle')
    console.log('new registered profile')
    console.log(my_member_profile)
    t.deepEqual(my_member_profile.Ok.name, '@philt3r', 'player1 only member')
    const conversation_1_address = await player1.call('chat', 'chat', 'start_conversation', convoHoloscape)
    await s.consistency()
    let members_in_conversation_result = await player1.call('chat', 'chat', 'get_members', {conversation_address: conversation_1_address.Ok})
    await s.consistency()
    t.deepEqual(members_in_conversation_result.Ok.length, 1, 'player1 only member')
    members_in_conversation_result.Ok.forEach(async (address) => {
      let member_profile = await player1.call('chat', 'chat', 'get_member_profile', {agent_address: address})
      console.log('*member_profile*')
      console.log(member_profile)
    })

    console.log(conversation_1_address)
    const post_result = await player1.call('chat', 'chat', 'post_message', {conversation_address: conversation_1_address.Ok, message: convoHoloscapeMessage1})
    await s.consistency()
    console.log(post_result)
    t.notEqual(post_result.Ok, undefined, 'post should return Ok')

    const {player2} = await s.players({player2: config2}, false)
    await player2.spawn()
    await create_persona_profile(s, t, player2, '@wollum', 'avatar', 'Willem Olding')
    let public_conversations_result = await player2.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(public_conversations_result.Ok)
    const holoscape_convo_address = public_conversations_result.Ok[0].address
    await player2.call('chat', 'chat', 'join_conversation', {conversation_address: holoscape_convo_address})
    await s.consistency()
    members_in_conversation_result = await player2.call('chat', 'chat', 'get_members', {conversation_address: holoscape_convo_address})
    await s.consistency()
    t.deepEqual(members_in_conversation_result.Ok.length, 2, 'player1 & player 2 members')
    members_in_conversation_result.Ok.forEach(async (address) => {
      let member_profile = await player2.call('chat', 'chat', 'get_member_profile', {agent_address: address})
      console.log('*member_profile*')
      console.log(member_profile)
    })
    const player2_post_result = await player2.call('chat', 'chat', 'post_message', {conversation_address: holoscape_convo_address, message: convoHoloscapeMessage2})
    await s.consistency()
    console.log(player2_post_result)
    t.notEqual(player2_post_result.Ok, undefined, 'post should return Ok')
    const get_message_result = await player2.call('chat', 'chat', 'get_messages', {address: holoscape_convo_address})
    await s.consistency()
    console.log(get_message_result)
    t.deepEqual(get_message_result.Ok.length, 2, 'a message from player1 and player2')

    const {player3} = await s.players({player3: config3}, false)
    await player3.spawn()
    await create_persona_profile(s, t, player3, 'Art', 'avatar', 'Arthur Brock')
    const player3_public_conversations_result = await player3.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(player3_public_conversations_result.Ok)
    const player3_holoscape_convo_address = player3_public_conversations_result.Ok[0].address
    await player3.call('chat', 'chat', 'join_conversation', {conversation_address: player3_holoscape_convo_address})
    await s.consistency()
    const player3_get_message_result = await player3.call('chat', 'chat', 'get_messages', {address: player3_holoscape_convo_address})
    await s.consistency()
    console.log(player3_get_message_result)
    t.deepEqual(player3_get_message_result.Ok.length, 2, 'Player 3 sees messages from player1 and player2')

    const player1_post_result_2 = await player1.call('chat', 'chat', 'post_message', {conversation_address: holoscape_convo_address, message: convoHoloscapeMessage3})
    await s.consistency()

    await player1.kill()

    const player3_get_message_result_2 = await player3.call('chat', 'chat', 'get_messages', {address: holoscape_convo_address})
    await s.consistency()
    console.log(player3_get_message_result_2)
    t.deepEqual(player3_get_message_result_2.Ok.length, 3, 'Player 3 sees 2 messages from player1 and 1 from player2')

    const {player4} = await s.players({player4: config4}, false)
    await player4.spawn()
    await create_persona_profile(s, t, player4, 'lucksus', 'avatar', 'Nicholas Luck')
    const player4_public_conversations_result = await player4.call('chat', 'chat', 'get_all_public_conversations', {})
    console.log(player4_public_conversations_result.Ok)
    const player4_holoscape_convo_address = player4_public_conversations_result.Ok[0].address
    await player4.call('chat', 'chat', 'join_conversation', {conversation_address: player4_holoscape_convo_address})
    await s.consistency()
    const player4_get_message_result = await player4.call('chat', 'chat', 'get_messages', {address: player4_holoscape_convo_address})
    await s.consistency()
    console.log('player4_get_message_result')
    console.log(player4_get_message_result)
    t.deepEqual(player4_get_message_result.Ok.length, 3, 'Player 4 sees 2 messages from player1 and 1 from player2')
    const player4_post_result = await player4.call('chat', 'chat', 'post_message', {conversation_address: player4_holoscape_convo_address, message: convoHoloscapeMessage4})
    await s.consistency()
    console.log('player4_post_result')
    console.log(player4_post_result)
    await player4.kill()

    //Bring player 1 back online
    await player1.spawn()
    await s.consistency()

    // Comment next 4 lines and test behaves differently
    // get members bit fails.
    // public_conversations_result = await player1.call('chat', 'chat', 'get_all_public_conversations', {})
    // console.log('public_conversations_result')
    // console.log(public_conversations_result)
    // t.deepEqual(public_conversations_result.Ok.length, 1, '1 Conversation')

    await player1.call('chat', 'chat', 'join_conversation', {conversation_address: holoscape_convo_address})
    await s.consistency()
    members_in_conversation_result = await player1.call('chat', 'chat', 'get_members', {conversation_address: holoscape_convo_address})
    console.log('members_in_conversation_result')
    console.log(members_in_conversation_result)
    await s.consistency()
    t.deepEqual(members_in_conversation_result.Ok.length, 4, 'player1,2,3 & 4 members') // fails when its 5 as the logic in teh hApp looks for existing links
    // however because     let all_member_ids = hdk::get_links(&address, LinkMatch::Exactly("has_member"), LinkMatch::Any)?.addresses().to_owned(); is coming back empty
    // the agent is added again.
    members_in_conversation_result.Ok.forEach(async (address) => {
      let member_profile = await player1.call('chat', 'chat', 'get_member_profile', {agent_address: address})
      console.log('*member_profile*')
      console.log(member_profile)
    })
    const player1_get_message_result = await player1.call('chat', 'chat', 'get_messages', {address: holoscape_convo_address})
    await s.consistency()
    console.log(player1_get_message_result)
    t.deepEqual(player1_get_message_result.Ok.length, 4, 'Player 1 sees 2 messages from player1 and 1 from player2 and 1 from Player 4')
  })
}
