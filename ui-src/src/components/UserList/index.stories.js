import React from 'react'
import { storiesOf } from '@storybook/react'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { specs } from 'storybook-addon-specifications'
import { UserList } from './index'
import { displayTests } from './display.test'
import { displayFullNameTests } from './displayFullName.test'


let props = {
  state: {
    room: {
      id: 'QmVQC9Fno8QFXgnW9i5tNdabcKxtTeXYXPeecK2qjn8bDx',
      private: true,
      name: 'Holochain Meetup - Melbourne',
      users: [
        'HcSCIUKodbWktcbmwyAaVrYnsbadi6b9sOV9D4ZoE8i333ZtcGw38Jn3U6u63qi',
        'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei'
      ]
    },
    users: {
      'HcSCIUKodbWktcbmwyAaVrYnsbadi6b9sOV9D4ZoE8i333ZtcGw38Jn3U6u63qi': {
        name: 'Art',
        avatar_url: 'https://avatars1.githubusercontent.com/u/117967?s=60&v=4',
        address: 'HcSCIUKodbWktcbmwyAaVrYnsbadi6b9sOV9D4ZoE8i333ZtcGw38Jn3U6u63qi'
      },
      'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei': {
        name: '@philt3r',
        avatar_url: 'https://avatars0.githubusercontent.com/u/5264862?s=60&v=4',
        address: 'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei'
      }
    }
  },
  actions: {
    setFirstName: userId => {
      console.log(userId)
      let user =  { ...props.state.users[userId], full_name: 'Full Name' }
      let users = { ...props.state.users, [userId]: user }
      store.set({
        state: { ...props.state, users }
      })
    }
  }
}

const store = new Store({
  state: props.state,
  actions: props.actions
})

storiesOf('User List', module)
  .addDecorator(StateDecorator(store))
  .add('Display', (() => {
    specs(() => displayTests)
    let user = {
      name: '@philt3r',
      avatar_url: 'https://avatars0.githubusercontent.com/u/5264862?s=60&v=4',
      address: 'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei'    }
    let users = { ...props.state.users, ['HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei']: user }
    store.set({
      state: { ...props.state, users }
    })
    return <UserList {...store.get('props')} />
  }))
  .add('Display with Full Name', (() => {
    specs(() => displayFullNameTests)
    let user = {
      name: '@philt3r',
      avatar_url: 'https://avatars0.githubusercontent.com/u/5264862?s=60&v=4',
      address: 'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei',
      full_name: 'Philip Beadle'
    }
    let users = { ...props.state.users, ['HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei']: user }
    store.set({
      state: { ...props.state, users }
    })
    return <UserList {...store.get('props')} />
  }))
