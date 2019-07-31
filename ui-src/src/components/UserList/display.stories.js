import React from 'react'
import { storiesOf } from '@storybook/react'
import { StateDecorator, Store } from '@sambego/storybook-state'
import { UserList } from './index'

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
        address: 'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei',
        first_name: 'Philip'
      }
    }
  },
  actions: {
    setFirstName: setFirstName => setFirstName()
  }
}

const store = new Store({
  props: props
})

function setFirstName () {
  let user = {
    name: '@philt3r',
    avatar_url: 'https://avatars0.githubusercontent.com/u/5264862?s=60&v=4',
    address: 'HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei',
    first_name: 'Philip'
  }

  store.set({
    props: { ...props.state.users, ['HcSCJbpm8CZparb6f5iEKKRv4Te4op93q6ESMRuDtwp9abbzV8zYTF7EKqqw9ei']: user }
  })
  console.log(store.get('props'))
}

storiesOf('User List', module)
  .addDecorator(StateDecorator(store))
  .add('Display', (() => {
    return <UserList {...props} />
  }))
