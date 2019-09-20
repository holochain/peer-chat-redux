import React from 'react'
import { storiesOf } from '@storybook/react'
import { action } from '@storybook/addon-actions'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { Group } from '../components/Group'

let props = {
  connected: true,
  groups: [{
    id: "group-address-1",
    name: "Public",
    icon: 'public'
  },
  {
    id: "group-address-2",
    name: "Private",
    icon: 'lock'
  },
  {
    id: "group-address-3",
    name: "Public",
    icon: 'public'
  }],
  user: {
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'hash address from holochain',
    firstName: '',
    lastName: ''
  },
  conversations: [{
    id: 'conversation_address',
    private: true,
    name: 'Ghost Mode',
    users: [{
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: '@philt3r',
      id: 'hash address from holochain',
      firstName: '',
      lastName: ''
    }]
  }],
  sidebarOpen: true,
  messages: [{
    id: 'message1',
    conversationId: 
    sender: 'QmAgentAddress',
    text: 'Peer Chat - Ghost Mode. Send p2p messages that aren\'t saved anywhere.',
    createdAt: Math.floor(Date.now() / 1000)
  },
  {
    id: 'message2',
    sender: 'QmAgentAddress2',
    text: 'Peer Chat - Ghost Mode. Send p2p messages that aren\'t saved anywhere.',
    createdAt: Math.floor(Date.now() / 1000)
  }],
  conversation: {
    id: 'conversation_address',
    private: true,
    name: 'Ghost Mode',
    users: [{
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: '@philt3r',
      id: 'QmAgentAddress',
      firstName: '',
      lastName: ''
    },
    {
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: 'Jarod',
      id: 'QmAgentAddress2',
      firstName: '',
      lastName: ''
    }]
  },
  joinGroup: action('Join Group'),
  getConversations: action('Get Conversations'),
  startConversation: action('Start Conversation'),
  joinConversation: action('Join Conversation'),
  setSidebar: sidebarOpen => {
      console.log('setSidebar')
      console.log(sidebarOpen)
      store.set({
        sidebarOpen: {sidebarOpen: sidebarOpen}
    })
  },
  sendMessage: action('Send Message'),
  runCommand: action('Run Command'),
  getMessages: action('Get Messages')
}

const store = new Store({
  props: props
});

storiesOf('Welcome', module)
  .addDecorator(StateDecorator(store))
  .add('Home Page', (() => {
    console.log(props.connected)
    store.set({
      props: {...props }
    })
    return <Group {...store.get('props')} />
  }))
