const { Config } = require('@holochain/try-o-rama')
const peer_chat_dnaPath = "./dist/dna-src.dna.json"
const personas_dnaPath = "../../identity-manager/dna/personas-profiles.dna.json"
const peer_chat_dna_1 = Config.dna(peer_chat_dnaPath, 'chat_1')
const personas_dna_1 = Config.dna(personas_dnaPath, 'personas_1', {uuid: 'agent1'})
const peer_chat_dna_2 = Config.dna(peer_chat_dnaPath, 'chat_2')

module.exports = {
  config1: {
    instances: {
      chat: peer_chat_dna_1,
      personas: personas_dna_1
    },
    bridges: [
      Config.bridge('p-p-bridge', 'chat', 'personas')
    ]
  },
  config2: {
    instances: {
      chat: peer_chat_dna_2
    }
  }
}
