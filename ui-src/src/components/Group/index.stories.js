import React from 'react'
import { storiesOf } from '@storybook/react'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { Group } from './index'

let conversation = {
  id: 'conversation_address',
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

let conversation_phil_nico = {
  id: 'conversation_phil_nico',
  private: true,
  group: 'Phil & Nico',
  name: 'Ghost Chat',
  users: [{
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    fullName: ''
  },
  {
    avatarURL: 'https://avatars1.githubusercontent.com/u/311749?s=60&v=4',
    name: 'Lucksus',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    fullName: ''
  }]
}

let conversation_phil_nico_music = {
  id: 'conversation_phil_nico_music',
  private: true,
  group: 'Phil & Nico',
  name: 'Music',
  users: [{
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    fullName: ''
  },
  {
    avatarURL: 'https://avatars1.githubusercontent.com/u/311749?s=60&v=4',
    name: 'Lucksus',
    id: 'HcScjTnefoi6c79eunbqfFNYEYovwaygbPkWEk95xVPd7vemvoB9Qwbjxf458ii',
    fullName: ''
  }]
}

let props = {
  user: {},
  users: [],
  conversations: [],
  sidebarOpen: false,
  messages: [],
  conversation: {},
  userListOpen: false,
  groups: [],
  getConversations: jest.fn(),
  startConversation: jest.fn(),
  joinConversation: jest.fn(),
  setSidebar: jest.fn(),
  sendMessage: jest.fn(),
  setUserList: jest.fn(),
  setFullName: jest.fn(),
  joinGroup: jest.fn()
}

const store = new Store({
  props: props
});

storiesOf('Chat Group', module)
  .addDecorator(StateDecorator(store))
  .add('No Profile Set', (() => {
    store.set({
      props: {user: {}}
    })
    return <Group {...store.get('props')} />
  }))
  .add('Profile Set', (() => {
    store.set({
      props: {
        connected: true,
        user: {
          avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
          name: '@philt3r',
          id: 'QmAgentAddress',
          fullName: ''
        },
        groups: [
          {
            id: 'peer-chat-public',
            name: 'Public',
            icon: '0x1F4E2'
          },
          {
            id: 'add',
            name: 'Add',
            icon: '10133'
          }
        ],
        currentGroup: {
          id: 'peer-chat-public',
          name: 'Public',
          icon: '128123'
        },
        conversations: [conversation],
        conversation: conversation
      }
    })
    return <Group {...store.get('props')} />
  }))
  .add('Chat Group Added', (() => {
    store.set({
      props: {
        connected: true,
        user: {
          avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
          name: '@philt3r',
          id: 'QmAgentAddress',
          fullName: ''
        },
        groups: [
          {
            id: 'peer-chat-public',
            name: 'Public',
            icon: '0x1F4E2'
          },
          {
            id: 'peer-chat-phil-nico',
            name: 'Phil & Nico',
            icon: '129302'
          },
          {
            id: 'add',
            name: 'Add',
            icon: '10133'
          }
        ],
        currentGroup: {
          id: 'peer-chat-phil-nico',
          name: 'Phil & Nico',
          icon: '129302'
        },
        conversations: [conversation_phil_nico],
        conversation: conversation_phil_nico
      }
    })
    return <Group {...store.get('props')} />
  }))
  .add('Conversation Added to Chat Group', (() => {
    store.set({
      props: {
        connected: true,
        user: {
          avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
          name: '@philt3r',
          id: 'QmAgentAddress',
          fullName: ''
        },
        groups: [
          {
            id: 'peer-chat-public',
            name: 'Public',
            icon: '0x1F4E2'
          },
          {
            id: 'peer-chat-phil-nico',
            name: 'Phil & Nico',
            icon: '129302'
          },
          {
            id: 'add',
            name: 'Add',
            icon: '10133'
          }
        ],
        currentGroup: {
          id: 'peer-chat-phil-nico',
          name: 'Phil & Nico',
          icon: '129302'
        },
        conversations: [conversation_phil_nico, conversation_phil_nico_music],
        conversation: conversation_phil_nico_music
      }
    })
    return <Group {...store.get('props')} />
  }))
