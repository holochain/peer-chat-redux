module.exports = scenario => {

  const testFieldSpec = {
    name: 'handle',
    displayName: 'Test Field',
    required: true,
    description: '',
    usage: 'STORE',
    schema: ''
  }

  const testProfileSpec = {
    name: 'something',
    sourceDna: 'xxx',
    fields: [testFieldSpec]
  }

  const personaAddress = 'QmVV3EiRGMj28u8CKtY1aKuMxYH4hMKF6YhXxS2PLWAbfN'

  scenario('Registers the Profile Spec when there is no existing profile', async (s, t, {chat_instance, personas_instance}) => {
    const get_profile_result_1 = await chat_instance.call('chat', 'get_my_member_profile', {})
    let sourceDna = get_profile_result_1
    console.log('sourceDna ' + JSON.stringify(sourceDna))
    const get_result = await personas_instance.call('profiles', 'get_profiles', {})
    console.log('Profiles' + JSON.stringify(get_result))
    t.deepEqual(get_result.Ok.length, 1)
  })

  scenario('Can register a profile and retrieve', async (s, t, {chat_instance}) => {
    const register_result = await chat_instance.call('chat', 'register', {name: 'alice', avatar_url: ''})
    console.log(register_result)
    t.equal(register_result.Ok.length, 63)

    const get_profile_result = await chat_instance.call('chat', 'get_member_profile', {agent_address: register_result.Ok})
    console.log(get_profile_result)
  })
}
