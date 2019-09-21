const path = require('path')
const tape = require('tape')
const { Orchestrator, Config, tapeExecutor, callSync } = require('@holochain/try-o-rama')
import * as constants from './config'
process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const peer_chat_dnaPath = "./dist/dna-src.dna.json"
const personas_dnaPath = "../../identity-manager/dna/personas-profiles.dna.json"

const peer_chat_dna_1 = Config.dna(peer_chat_dnaPath, 'chat_1')
const personas_dna_1 = Config.dna(personas_dnaPath, 'personas_1', {uuid: 'agent1'})
const peer_chat_dna_2 = Config.dna(peer_chat_dnaPath, 'chat_2')


const orchestrator = new Orchestrator({
  globalConfig: {logger: false},
  middleware: combine(callSync, tapeExecutor(require('tape'))),
})

// require('./agent/profile')(orchestrator.registerScenario)
// require('./agent/messages')(orchestrator.registerScenario)

require('./scenario/full_name')(orchestrator.registerScenario)

orchestrator.run()
