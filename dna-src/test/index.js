const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const chat_dnaPath = "./dist/dna-src.dna.json"
const personas_dnaPath = "../../identity-manager/dna/personas-profiles.dna.json"

const chat_dna_1 = Diorama.dna(chat_dnaPath, 'chat_1')
const personas_dna_1 = Diorama.dna(personas_dnaPath, 'personas_1', {uuid: 'agent1'})
const chat_dna_2 = Diorama.dna(chat_dnaPath, 'chat_2')

const diorama_1 = new Diorama({
  instances: {
    chat_instance_1: chat_dna_1,
    personas_instance_1: personas_dna_1
  },
  bridges: [
    Diorama.bridge('p-p-bridge', 'chat_instance_1', 'personas_instance_1')
  ],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

const diorama_2 = new Diorama({
  instances: {
    chat_instance_1: chat_dna_1,
    personas_instance_1: personas_dna_1,
    chat_instance_2: chat_dna_2
  },
  bridges: [
    Diorama.bridge('p-p-bridge', 'chat_instance_1', 'personas_instance_1')
  ],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

require('./agent/profile')(diorama_1.registerScenario)
require('./agent/messages')(diorama_1.registerScenario)

require('./scenario/full_name')(diorama_2.registerScenario)

diorama_1.run()
diorama_2.run()
