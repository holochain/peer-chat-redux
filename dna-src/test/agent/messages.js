module.exports = scenario => {

  const testMessage = {
    timestamp: 0,
    message_type: "text",
    payload: "I am the message payload",
    meta: "{}",
  }

  const testNewChannelParams = {
    name: "test new conversation",
    description: "for testing...",
    initial_members: [],
    public: true
  }

  scenario('Can post a message to the conversation and retrieve', async (s, t, {chat_instance_1}) => {

    const register_result = await chat_instance_1.call('chat', 'register', {name: 'chat_instance_1', avatar_url: ''})
    console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const create_result = await chat_instance_1.call('chat', 'create_conversation', testNewChannelParams)
    console.log(create_result)
    const conversation_addr = create_result.Ok
    t.deepEqual(conversation_addr.length, 46)

    const post_result = await chat_instance_1.call('chat', 'post_message', {conversation_address: conversation_addr, message: testMessage})
    console.log(post_result)
    t.notEqual(post_result.Ok, undefined, 'post should return Ok')

    const get_message_result = await chat_instance_1.call('chat', 'get_messages', {address: conversation_addr})
    console.log(get_message_result)
    t.deepEqual(get_message_result.Ok[0].entry.payload, testMessage.payload, 'expected to receive the message back')
  })
}
