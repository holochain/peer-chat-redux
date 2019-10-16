const path = require('path')
const tape = require('tape')
const { Orchestrator, combine, callSync, singleConductor, tapeExecutor } = require('@holochain/try-o-rama')
const orchestrator = new Orchestrator({
  globalConfig: {logger: false, network: 'memory'},
  middleware: combine(callSync, singleConductor, tapeExecutor(tape))
})
process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

// require('./agent/profile')(orchestrator.registerScenario)
// require('./agent/messages')(orchestrator.registerScenario)

require('./scenario/full_name')(orchestrator.registerScenario)

orchestrator.run()
