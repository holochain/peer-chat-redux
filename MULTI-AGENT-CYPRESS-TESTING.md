# How to run the multi conductor testing

The setup will run 2 conductors each with a separate agent. Agent 2 will use the built in webserver and the config file uses a UI Bundle. Agent 1 however will use node to run the UI as the built in web server returns a null for content type and thus Cypress cannot automate it. This means we will first build the UI with no connection string for the web socket so that it picks up the connection from the conductor-config, then we will comment out the connection and use the line of code that defines the connections string so when we run it from ``` npm run start``` it can connect.
A little convoluted but works for now.

## Get both Agents UI and Conductors running.

- clone the repo
```
  git clone git@github.com:holochain/peer-chat.git
```

- install node modules for the UI and build to the ui folder for Agent 2.
```
  cd ui-src
  npm install
  npm run build
  cd ..
```

- get Agent 1 sorted.
  - Open ui-src/src/index.js
  - Comment out line 26
  - Uncomment line 25

- build the dna and run both conductors. There are included keys for 2 agents and 2 conductor-configs to do this.
```
  npm run hc:build
  npm run hc:start:agent1

  # open a new terminal tab for the second conductor

  npm run hc:start:agent2
```

  - agent 2 UI is now running at http://localhost:3002 and the http interface is at http://localhost:4002
  - agent 1 http interface is at http://localhost:4001
- run the agent 1 UI
```
  cd ui-src
  npm run start
```
  - agent 1 UI is now running at http://localhost:3000

## Get the automation running.

- install node modules for the automation and open Cypress
```
  cd ui-automation
  npm install
  npm run cypress:open
```

  - you can now see the 2 files of tests, register.js and conversations.js. These tests are designed to be run over and over again.
- Click the run all tests button.

### Tests

The tests are e2e that use either the http interface to POST a json RPC call or use the UI. The code for the tests is in the ui-automation/cypress/integration folder
