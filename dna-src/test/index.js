const path = require('path')
const tape = require('tape')
const { Orchestrator, combine, callSync, tapeExecutor } = require('@holochain/try-o-rama')
//hc-sim2h-server-install
//sim2h_server -p 9001
const orchestrator = new Orchestrator({
  globalConfig: {logger: true,  network: {
      type: 'sim2h',
      sim2h_url: 'wss://0.0.0.0:9001'
    }
  },
  middleware: combine(callSync, tapeExecutor(tape))
})
process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

// require('./agent/profile')(orchestrator.registerScenario)
// require('./agent/messages')(orchestrator.registerScenario)
//
// require('./scenario/full_name')(orchestrator.registerScenario)
require('./scenario/4-agents')(orchestrator.registerScenario)

orchestrator.run()
