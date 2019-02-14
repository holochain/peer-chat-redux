const { Config, Container, Scenario } = require('@holochain/holochain-nodejs')
Scenario.setTape(require('tape'))
const dnaPath = "dist/bundle.json"
const dna = Config.dna(dnaPath, 'happs')
const agentAlice = Config.agent("alice")
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

/*----------  Chat  ----------*/


const testNewChannelParams = {
  name: "test new stream",
  description: "for testing...",
  initial_members: [],
  public: true
}

const testMessage = {
  message_type: "text",
  payload: "I am the message payload",
  meta: "{}",
}

scenario.runTape('Can create a public stream with no other members and retrieve it', (t, {alice}) => {
 
  const get_all_members_result = alice.call('chat', 'get_all_members', {})
  console.log('all members:', get_all_members_result)
  let allMembers = get_all_members_result.Ok
  t.true(allMembers.length > 0, 'gets at least one member')

  const create_result = alice.call('chat', 'create_stream', testNewChannelParams)
  console.log(create_result)
  t.deepEqual(create_result.Ok.length, 46)

  const get_result = alice.call('chat', 'get_my_streams', {})
  console.log(get_result)
  t.deepEqual(get_result.Ok.length, 1)

})

scenario.runTape('Can retrieve all the members that are added by init', (t, {alice}) => {

  const getAllMembersResult = alice.call('chat', 'get_all_members', {})
  console.log(getAllMembersResult.Ok)
  t.equal(getAllMembersResult.Ok.length, 1) // will fail if we change test data
})

scenario.runTape('Can post a message to the stream and retrieve', (t, {alice}) => {

  const create_result = alice.call('chat', 'create_stream', testNewChannelParams)
  console.log(create_result)
  const stream_addr = create_result.Ok
  t.deepEqual(stream_addr.length, 46)

  const get_result = alice.call('chat', 'get_my_streams', {})
  console.log(get_result)
  t.deepEqual(get_result.Ok.length, 1)

  const post_result = alice.call('chat', 'post_message', {stream_address: stream_addr, message: testMessage, subjects: []})
  console.log(post_result)
  t.notEqual(post_result.Ok, undefined, 'post should return Ok')

  const get_message_result = alice.call('chat', 'get_messages', {address: stream_addr})
  console.log(get_message_result)
  t.deepEqual(get_message_result.Ok[0].entry.payload, testMessage.payload, 'expected to receive the message back')
})


scenario.runTape('Can post a message with a subject and this is added to the stream', (t, {alice}) => {

  const create_result = alice.call('chat', 'create_stream', testNewChannelParams)
  console.log(create_result)
  const stream_addr = create_result.Ok
  t.deepEqual(stream_addr.length, 46)

  const post_result = alice.call('chat', 'post_message', {stream_address: stream_addr, message: testMessage, subjects: ['subject 1', 'subject 2']})
  console.log(post_result)
  t.notEqual(post_result.Ok, undefined, 'post should return success')

  const get_subjects_result = alice.call('chat', 'get_subjects', {stream_address: stream_addr})
  console.log(get_subjects_result)
  t.deepEqual(get_subjects_result.Ok.length, 2)
  t.deepEqual(get_subjects_result.Ok[0].entry.stream_address.length, 46)
  t.deepEqual(get_subjects_result.Ok[0].address.length, 46)

  const get_subject_message_result = alice.call('chat', 'get_messages', {address: get_subjects_result.Ok[0].address})
  console.log('Messages linked to the subject' + get_subjects_result.Ok[0].address)
  t.deepEqual(get_subject_message_result.Ok[0].entry.payload, testMessage.payload, 'expected to receive the message back')


})

scenario.runTape('Can create a public stream with some members', (t, {alice}) => {

  const get_all_members_result = alice.call('chat', 'get_all_members', {})
  console.log('all members:', get_all_members_result)
  let allMemberAddrs = get_all_members_result.Ok.map(elem => elem.address)
  t.true(allMemberAddrs.length > 0, 'gets at least one member')

  const create_result = alice.call('chat', 'create_stream', {...testNewChannelParams, public: false, initial_members: allMemberAddrs})
  console.log(create_result)
  t.deepEqual(create_result.Ok.length, 46)
})


