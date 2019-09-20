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

  scenario('Can post a message to the conversation and retrieve', async (s, t, {chat_instance_1}) => {

    const register_result = await chat_instance_1.call('chat', 'register', {name: 'chat_instance_1', avatar_url: ''})
    // console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const create_result = await chat_instance_1.call('chat', 'create_stream', testNewChannelParams)
    // console.log(create_result)
    const stream_addr = create_result.Ok
    t.deepEqual(stream_addr.length, 46)

    const post_result = await chat_instance_1.call('chat', 'post_message', {stream_address: stream_addr, message: testMessage})
    // console.log(post_result)
    t.notEqual(post_result.Ok, undefined, 'post should return Ok')

    const get_message_result = await chat_instance_1.call('chat', 'get_messages', {address: stream_addr})
    // console.log(get_message_result)
    t.deepEqual(get_message_result.Ok[0].entry.payload, testMessage.payload, 'expected to receive the message back')
  })

  scenario('A message must have 1-1024 characters', async (s, t, {chat_instance_1}) => {

    const register_result = await chat_instance_1.call('chat', 'register', {name: 'chat_instance_1', avatar_url: ''})
    // console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const create_result = await chat_instance_1.call('chat', 'create_stream', testNewChannelParams)
    // console.log(create_result)
    const stream_addr = create_result.Ok
    t.deepEqual(stream_addr.length, 46)

    const post_result = await chat_instance_1.call('chat', 'post_message', {stream_address: stream_addr, message: testMessage_empty})
    // console.log(post_res ult)
    t.notEqual(post_result.Err, undefined, 'Message must have 1-1024 characters')

    const post_result_2 = await chat_instance_1.call('chat', 'post_message', {stream_address: stream_addr, message: testMessage_1024})
    // console.log(post_result_2)
    t.notEqual(post_result.Err, undefined, 'Message must have 1-1024 characters')

    const get_message_result = await chat_instance_1.call('chat', 'get_messages', {address: stream_addr})
    // console.log(get_message_result)
    t.deepEqual(get_message_result.Ok.length, 0, 'Messages should not be stored.')
  })
}
