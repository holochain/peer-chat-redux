import React from 'react'

import { storiesOf } from '@storybook/react'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { UserHeader } from './index'

let props = {
  state: {
    holochainConnection: null, // Use for debug
    connected: false,
    user: {
      avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      name: '@philt3r',
      id: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
      firstName: 'Philip',
      lastName: 'Beadle'
    },
    users: {},
    room: {},
    rooms: [],
    messages: {},
    openFullName: false,
    userListOpen: window.innerWidth > 1000,
    profileSpecSourceDna: ''
  },
  actions: {
    openFullName: openFullName => setUser(),
  }
}

const store = new Store({
  props: props
});

function setUser () {
  store.set({
    props: { ...props.state.user, firstName: 'Philip' }
  })
  console.log(store.get('props'))
}

storiesOf('User Header', module)
  .addDecorator(StateDecorator(store))
  .add('Display - User', (() => {
    return <UserHeader {...props} />
  }))
  .add('Display - No User', (() => {
    let newState = {...store.get('props').state, user: {}}
    console.log(newState)
    store.set({
      props: {...props, state: newState}
    })
    console.log(store.get('props'))
    let newProps = store.get('props')
    return <UserHeader {...newProps} />
  }))
