import React from 'react'

import { storiesOf } from '@storybook/react'
import { UserHeader } from './index'

let props = {
  state: {
    holochainConnection: null, // Use for debug
    connected: false,
    user: {
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: 'Philip',
      id: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4'
    },
    users: {},
    room: {},
    rooms: [],
    messages: {},
    sidebarOpen: false,
    userListOpen: window.innerWidth > 1000,
    profileSpecSourceDna: ''
  },
  actions: {}
}

storiesOf('User Header', module)
  .add('Display - User', (() => {
    return <UserHeader {...props} />
  }))
  .add('Display - No User', (() => {
    let localProps = JSON.parse(JSON.stringify(props))
    localProps.state.user = {}
    return <UserHeader {...localProps} />
  }))
