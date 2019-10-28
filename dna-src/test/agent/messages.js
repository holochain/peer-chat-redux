module.exports = scenario => {

  const testMessage = {
    timestamp: 0,
    message_type: "text",
    payload: "I am the message payload",
    meta: "{}",
  }

  const testMessage_empty = {
    timestamp: 0,
    message_type: "text",
    payload: "",
    meta: "{}",
  }

  const testMessage_1024 = {
    timestamp: 0,
    message_type: "text",
    payload: `aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    aaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaaaaa aaa aaaaaaaaa
    1`,
    meta: "{}",
  }


  const testNewChannelParams = {
    name: "test new conversation",
    description: "for testing...",
    initial_members: [],
    public: true
  }

  const { config1 } = require('../config')

  scenario('General Chat conversation created by default', async (s, t) => {
    const {player1} = await s.players({player1: config1}, true)
    const register_result = await player1.call('chat', 'chat', 'register', {name: 'player1', avatar_url: ''})
    await s.consistency()
    console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const default_conversations = await player1.call('chat', 'chat', 'get_all_public_conversations', {})
    await s.consistency()
    console.log(default_conversations)
    t.deepEqual(default_conversations.Ok.length, 1)
  })

  scenario('Can post a message to the conversation and retrieve', async (s, t) => {
    const {player1} = await s.players({player1: config1}, true)
    const register_result = await player1.call('chat', 'chat', 'register', {name: 'player1', avatar_url: ''})
    await s.consistency()
    console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const create_result = await player1.call('chat', 'chat', 'start_conversation', testNewChannelParams)
    await s.consistency()
    console.log(create_result)
    const conversation_addr = create_result.Ok
    t.deepEqual(conversation_addr.length, 46)

    const post_result = await player1.call('chat', 'chat', 'post_message', {conversation_address: conversation_addr, message: testMessage})
    await s.consistency()
    console.log(post_result)
    t.notEqual(post_result.Ok, undefined, 'post should return Ok')

    const get_message_result = await player1.call('chat', 'chat', 'get_messages', {address: conversation_addr})
    await s.consistency()
    console.log(get_message_result)
    t.deepEqual(get_message_result.Ok[0].entry.payload, testMessage.payload, 'expected to receive the message back')
  })

  scenario('A message must have 1-1024 characters', async (s, t) => {
    const {player1} = await s.players({player1: config1}, true)
    const register_result = await player1.call('chat', 'chat', 'register', {name: 'player1', avatar_url: ''})
    await s.consistency()
    // console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const create_result = await player1.call('chat', 'chat', 'start_conversation', testNewChannelParams)
    await s.consistency()
    // console.log(create_result)
    const conversation_addr = create_result.Ok
    t.deepEqual(conversation_addr.length, 46)

    const post_result = await player1.call('chat', 'chat', 'post_message', {conversation_address: conversation_addr, message: testMessage_empty})
    await s.consistency()
    // console.log(post_res ult)
    t.notEqual(post_result.Err, undefined, 'Message must have 1-1024 characters')

    const post_result_2 = await player1.call('chat', 'chat', 'post_message', {conversation_address: conversation_addr, message: testMessage_1024})
    await s.consistency()
    // console.log(post_result_2)
    t.notEqual(post_result.Err, undefined, 'Message must have 1-1024 characters')

    const get_message_result = await player1.call('chat', 'chat', 'get_messages', {address: conversation_addr})
    // console.log(get_message_result)
    t.deepEqual(get_message_result.Ok.length, 0, 'Messages should not be stored.')
  })
}
