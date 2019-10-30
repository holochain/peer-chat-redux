import React from 'react'
import { storiesOf } from '@storybook/react'
import { action } from '@storybook/addon-actions'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { Group } from '../components/Group'

let conversation = {
  id: 'public_conversation',
  private: true,
  group: 'Public',
  name: 'Ghost Chat',
  users: [{
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    fullName: ''
  }]
}

let props = {
  connected: true,
  groups: [{
    id: "peer-chat-public",
    name: "Public",
    icon: '0x1F4E2'
  },
  {
    id: "group-address-2",
    name: "Private",
    icon: '129302'
  },
  {
    id: "group-address-3",
    name: "Public",
    icon: '0x1F354'
  },
  {
    id: 'add',
    name: 'Add',
    icon: '10133'
  }],
  currentGroup: {
    id: "peer-chat-public",
    name: "Public",
    icon: 'public'
  },
  user: {
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    firstName: '',
    lastName: ''
  },
  conversations: [conversation],
  sidebarOpen: true,
  messages: [{
    id: 'QmbBR5wvLoXdbEdX7GvU8diBHbbojehtbNMWCvsPprb1iE',
    sender: 'QmAgentAddress',
    text: 'Peer Chat - Ghost Mode. Send p2p messages that aren\'t saved anywhere.',
    createdAt: Math.floor(Date.now() / 1000)
  },
  {
    id: 'QmYgEY1UqntPsn14vxHiEqW5Xw99YajpRNhPwxEdaVXSdz',
    sender: 'QmAgentAddress',
    text: 'Peer Chat - Ghost Mode',
    createdAt: Math.floor(Date.now() / 1000)
  }],
  conversation: conversation,
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
    store.set({
      props: {...props }
    })
    return <Group {...store.get('props')} />
  }))
