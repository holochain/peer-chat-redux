module.exports = scenario => {

  const testFieldSpec = {
    name: "handle",
    displayName: "Test Field",
    required: true,
    description: "",
    usage: "STORE",
    schema: ""
  }

  const testProfileSpec = {
    name: "something",
    sourceDna: "xxx",
    fields: [testFieldSpec]
  }

  scenario('Registers the Profile Spec when there is no existing profile', async (s, t, {chat_instance, personas_instance}) => {
    const get_profile_result_1 = await chat_instance.call('chat', 'get_my_member_profile', {})
    let sourceDna = get_profile_result_1
    console.log('sourceDna ' + JSON.stringify(sourceDna))
    //
    // const get_profile_result_2 = await chat_instance.callSync('chat', 'get_member_profile', {agent_address: 'HcScjwO9ji9633ZYxa6IYubHJHW6ctfoufv5eq4F7ZOxay8wR76FP4xeG9pY3ui'})
    // sourceDna = get_profile_result_2.Err.Internal
    // console.log('sourceDna ' + sourceDna)

    //
    // const register_result = await personas_instance.call("profiles", "register_app", {spec: testProfileSpec})
    // console.log(register_result)
    // t.notEqual(register_result.Ok, undefined)

    const get_result = await personas_instance.call("profiles", "get_profiles", {})
    console.log("Profiles" + JSON.stringify(get_result))
    t.deepEqual(get_result.Ok.length, 1)

  })
}
