import React from 'react'
import { storiesOf } from '@storybook/react'
import { action } from '@storybook/addon-actions'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { Group } from '../components/Group'

let props = {
  groups: [{
    id: "group-address",
    name: "Public"
  }],
  user: {
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'hash address from holochain',
    firstName: '',
    lastName: ''
  },
  conversations: [{
    id: 'address',
    private: true,
    name: 'Story 123',
    users: [{
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: '@philt3r',
      id: 'hash address from holochain',
      firstName: '',
      lastName: ''
    }]
  }],
  sidebarOpen: true,
  messages: [],
  conversation: {},
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
    console.log(props.sidebarOpen)
    store.set({
      props: {...props }
    })
    return <Group {...store.get('props')} />
  }))
