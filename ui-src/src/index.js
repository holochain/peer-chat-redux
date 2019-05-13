import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import { connect } from '@holochain/hc-web-client'

import { UserHeader } from './components/UserHeader'
import { UserList } from './components/UserList'
import { MessageList } from './components/MessageList'
import { CreateMessageForm } from './components/CreateMessageForm'
import { RoomList } from './components/RoomList'
import { RoomHeader } from './components/RoomHeader'
import { CreateRoomForm } from './components/CreateRoomForm'
import { WelcomeScreen } from './components/WelcomeScreen'
import { JoinRoomScreen } from './components/JoinRoomScreen'
import { RegisterScreen } from './components/RegisterScreen'

// --------------------------------------
// Application
// --------------------------------------

class View extends React.Component {
  constructor (props) {
    super(props)
    this.state = {
      holochainConnection: connect(), // Use for debug
      // holochainConnection: connect(), // use when letting the conductor auto-select. Allows for multiple agents
      connected: false,
      user: {},
      users: {},
      room: {},
      rooms: [],
      messages: {},
      sidebarOpen: false,
      userListOpen: window.innerWidth > 1000
    }

    this.actions = {
      // --------------------------------------
      // UI
      // --------------------------------------

      setSidebar: sidebarOpen => this.setState({ sidebarOpen }),
      setUserList: userListOpen => this.setState({ userListOpen }),

      // --------------------------------------
      // User
      // --------------------------------------

      setUser: user => {
        this.setState({ user })
        this.actions.getRooms()
      },

      // --------------------------------------
      // Room
      // --------------------------------------

      setRoom: room => {
        this.setState({ room, sidebarOpen: false })
        this.actions.getMessages(room.id)
        this.actions.getRoomMembers(room.id)
        this.actions.scrollToEnd()
      },

      joinRoom: room => {
        console.log('joining room')
        this.actions.setRoom(room)
        this.makeHolochainCall('holo-chat/chat/join_stream', { stream_address: room.id }, (result) => {
          console.log('joined room', result)
        })
      },

      getRoomMembers: roomId => {
        this.makeHolochainCall('holo-chat/chat/get_members', {
          stream_address: roomId
        }, (result) => {
          console.log('retrieved members', result)
          const users = result.Ok
          users.forEach(address => {
            this.actions.getUserProfile(address)
          })
          this.setState({
            room: { ...this.state.room, users }
          })
        })
      },

      sendMessage: ({ text, roomId }) => {
        const message = {
          message_type: 'text',
          timestamp: Math.floor(Date.now() / 1000),
          payload: text,
          meta: ''
        }
        this.makeHolochainCall('holo-chat/chat/post_message', {
          stream_address: roomId,
          message
        }, (result) => {
          console.log('message posted', result)
          setTimeout(() => this.actions.getMessages(roomId), 1000) // hack for now
          this.actions.scrollToEnd()
        })
      },

      getMessages: (roomId) => {
        this.makeHolochainCall('holo-chat/chat/get_messages', { address: roomId }, (result) => {
          console.log('retrieved messages', result)

          const roomMessages = result.Ok.map(({ address, entry }) => ({
            text: entry.payload,
            sender: entry.author,
            createdAt: entry.timestamp,
            id: address
          }))

          this.setState({
            messages: { ...this.state.messages, [roomId]: roomMessages }
          })
        })
      },

      createRoom: options => {
        console.log(options)
        const roomSpec = {
          name: options.name,
          description: '',
          initial_members: []
        }
        this.makeHolochainCall('holo-chat/chat/create_stream', roomSpec, (result) => {
          console.log('created room', result)
          this.actions.setRoom({
            id: result.Ok,
            name: options.name,
            users: []
          })
          this.actions.getRooms()
        })
      },

      getUserProfile: userId => {
        this.makeHolochainCall('holo-chat/chat/get_member_profile', { agent_address: userId }, (result) => {
          console.log('retrieved profile', result)
          this.setState({
            users: { ...this.state.users, [userId]: result.Ok }
          })
        })
      },

      getRooms: () => {
        this.makeHolochainCall('holo-chat/chat/get_all_public_streams', {}, (result) => {
          console.log('retrieved public rooms', result)
          let rooms = result.Ok.map(({ address, entry }) => {
            return {
              id: address,
              private: !entry.public,
              name: entry.name,
              users: []
            }
          })
          this.setState({
            rooms
          })
        })
      },

      registerUser: ({ name, avatarURL }) => {
        this.makeHolochainCall('holo-chat/chat/register', { name, avatar_url: avatarURL }, result => {
          console.log('registered user', result)
          this.actions.setUser({ id: result.Ok, name, avatarURL })
        })
      },

      scrollToEnd: e =>
        setTimeout(() => {
          const elem = document.querySelector('#messages')
          elem && (elem.scrollTop = 100000)
        }, 0)

    }
  }

  componentDidMount () {
    this.state.holochainConnection.then(({ callZome }) => {
      callZome('holo-chat', 'chat', 'get_my_member_profile')({}).then((result) => {
        const profile = JSON.parse(result).Ok
        if (profile) {
          console.log('registration user found with profile:', profile)
          this.actions.setUser({ id: profile.address, name: profile.name, avatarURL: profile.avatar_url })
        } else {
          console.log('User has not registered a profile. Complete the form to proceed')
        }
        this.setState({ connected: true })
      })
    })
  }

  makeHolochainCall (callString, params, callback) {
    const [instanceId, zome, func] = callString.split('/')
    this.state.holochainConnection.then(({ callZome }) => {
      callZome(instanceId, zome, func)(params).then((result) => callback(JSON.parse(result)))
    })
  }

  render () {
    const {
      user,
      users,
      room,
      rooms,
      messages,
      sidebarOpen,
      userListOpen,
      connected
    } = this.state
    const { createRoom, registerUser } = this.actions

    return (
      <main>
        <aside data-open={sidebarOpen}>
          <UserHeader user={user} />
          <RoomList
            user={user}
            rooms={rooms}
            messages={messages}
            current={room}
            actions={this.actions}
          />
          {user.id && <CreateRoomForm submit={createRoom} />}
        </aside>
        <section>
          <RoomHeader state={this.state} actions={this.actions} />
          {room.id ? (
            <row->
              <col->
                <MessageList
                  user={user}
                  users={users}
                  messages={messages[room.id]}
                />
                <CreateMessageForm state={this.state} actions={this.actions} />
              </col->
              {userListOpen && (
                <UserList
                  room={room}
                  current={user.id}
                  users={users}
                />
              )}
            </row->
          ) : connected ? (
            user.id ? <JoinRoomScreen /> : <RegisterScreen registerUser={registerUser} />
          ) : (
            <WelcomeScreen message='Connecting to Holochain... Make sure the conductor is running and try refreshing the page' />
          )}
        </section>
      </main>
    )
  }
}

ReactDOM.render(<View />, document.querySelector('#root'))
