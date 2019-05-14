import '../support/addAgents'

describe('Create a room via UI and API', function() {

  it('Agent 1 create a room with the UI', function() {
    cy.visit('http://localhost:3000/')
    cy.get('#room').type('Room 1 Created by Agent 1')
    cy.get('#submit').click()

    cy.contains('philip')
  })

  it('Agent 2 reate a room with the API check Agent 1 UI can see it', function() {
    var message = {
      "jsonrpc": "2.0",
      "id": 2,
      "method": "call",
      "params": {
          "instance_id": "holo-chat",
          "zome": "chat",
          "function": "create_stream",
          "params": { "name": "Room 3 Created by Agent 2", "description": "Another room", "initial_members": [] }
      }
    }
    cy.request('POST', 'http://localhost:4002', message)
    cy.wait(5000)
    cy.visit('http://localhost:3000/')
    cy.get('#refresh').click()
    cy.contains('Room 3 Created by Agent 2')
  })
})
