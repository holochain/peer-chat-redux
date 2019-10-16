import React from 'react'

import { storiesOf } from '@storybook/react'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { UserHeader } from './index'

let props = {
  user: {
    avatarURL: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    name: '@philt3r',
    id: 'https://avatars3.githubusercontent.com/u/5264862?s=40&v=4',
    firstName: '',
    lastName: ''
  }
}

const store = new Store({
  props: props
});

storiesOf('User Header', module)
  .addDecorator(StateDecorator(store))
  .add('Display - User', (() => {
    store.set({
      props: {user: props.user}
    })
    return <UserHeader {...store.get('props')} />
  }))
  .add('Display - No User', (() => {
    store.set({
      props: {user: {}}
    })
    console.log(store.get('props'))
    return <UserHeader {...store.get('props')} />
  }))
