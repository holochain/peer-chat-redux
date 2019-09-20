import '../support/addAgents'

describe('Create a conversation via UI and API', function() {

  it('Agent 1 create a conversation with the UI', function() {
    cy.visit('http://localhost:3000/')
    cy.get('#conversation').type('Conversation 1 Created by Agent 1')
    cy.get('#submit').click()

    cy.contains('philip')
  })

  it('Agent 2 reate a conversation with the API check Agent 1 UI can see it', function() {
    var message = {
      "jsonrpc": "2.0",
      "id": 2,
      "method": "call",
      "params": {
          "instance_id": "peer-chat",
          "zome": "chat",
          "function": "create_conversation",
          "params": { "name": "Conversation 3 Created by Agent 2", "description": "Another conversation", "initial_members": [] }
      }
    }
    cy.request('POST', 'http://localhost:4002', message)
    cy.visit('http://localhost:3000/')
    cy.get('#refresh').click()
    cy.contains('Conversation 3 Created by Agent 2')
  })
})
