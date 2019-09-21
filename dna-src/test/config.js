export const config1 = {
  instances: {
    chat: peer_chat_dna_1,
    personas: personas_dna_1
  },
  bridges: [
    Orchestrator.bridge('p-p-bridge', 'chat_instance_1', 'personas_instance_1')
  ]
}

export const config2 = {
  instances: {
    chat: peer_chat_dna_2
  },
}
