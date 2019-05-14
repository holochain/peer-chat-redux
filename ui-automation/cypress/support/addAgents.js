before(function(){
  var agent1 = {
    "jsonrpc": "2.0",
    "id": 2,
    "method": "call",
    "params": {
        "instance_id": "holo-chat",
        "zome": "chat",
        "function": "register",
        "params": { "name": "philip", "avatar_url": "" }
    }
  }
  cy.request('POST', 'http://localhost:4001', agent1)

  var agent2 = {
    "jsonrpc": "2.0",
    "id": 2,
    "method": "call",
    "params": {
        "instance_id": "holo-chat",
        "zome": "chat",
        "function": "register",
        "params": { "name": "willem", "avatar_url": "" }
    }
  }
  cy.request('POST', 'http://localhost:4002', agent2)
})
