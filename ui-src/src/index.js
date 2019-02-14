import React from 'react'
import ReactDOM from 'react-dom'
import { set } from 'object-path-immutable'
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

// --------------------------------------
// Application
// --------------------------------------

class View extends React.Component {
  state = {
    holochainConnection: connect("ws://localhost:3400"),
    user: {},
    room: {},
    rooms: [],
    messages: {},
    sidebarOpen: false,
    userListOpen: window.innerWidth > 1000,
  }

  actions = {
    // --------------------------------------
    // UI
    // --------------------------------------

    setSidebar: sidebarOpen => this.setState({ sidebarOpen }),
    setUserList: userListOpen => this.setState({ userListOpen }),

    // --------------------------------------
    // User
    // --------------------------------------

    setUser: user => this.setState({ user }),

    // --------------------------------------
    // Room
    // --------------------------------------

    setRoom: room => {
      this.setState({ room, sidebarOpen: false })
      this.actions.scrollToEnd()
    },

    joinRoom: room => {
      this.actions.setRoom(room)
      this.actions.subscribeToRoom(room)
      this.state.messages[room.id] &&
        this.actions.setCursor(
          room.id,
          Object.keys(this.state.messages[room.id]).pop()
        )
    },

    subscribeToRoom: room =>
      !this.state.user.roomSubscriptions[room.id] &&
      this.state.user.subscribeToRoom({
        roomId: room.id,
        hooks: { onMessage: this.actions.addMessage },
      }),

    createRoom: options => {
      console.log(options)
      const roomSpec = {
        name: options.name,
        description: '',
        initial_members: [],
        public: !options.private
      }
      this.makeHolochainCall('holo-chat/chat/create_stream', roomSpec, (result) => {
        console.log(result)
        this.actions.setRoom({
          id: result.Ok,
          name: options.name
        })
      })
    },

    createConvo: options => {
      if (options.user.id !== this.state.user.id) {
        const exists = this.state.user.rooms.find(
          x =>
            x.name === options.user.id + this.state.user.id ||
            x.name === this.state.user.id + options.user.id
        )
        exists
          ? this.actions.joinRoom(exists)
          : this.actions.createRoom({
              name: this.state.user.id + options.user.id,
              addUserIds: [options.user.id],
              private: true,
            })
      }
    },

    addUserToRoom: ({ userId, roomId = this.state.room.id }) =>
      this.state.user
        .addUserToRoom({ userId, roomId })
        .then(this.actions.setRoom),


    // --------------------------------------
    // Cursors
    // --------------------------------------

    setCursor: (roomId, position) =>
      this.state.user
        .setReadCursor({ roomId, position: parseInt(position) })
        .then(x => this.forceUpdate()),

    // --------------------------------------
    // Messages
    // --------------------------------------

    addMessage: payload => {
      const roomId = payload.room.id
      const messageId = payload.id
      // Update local message cache with new message
      this.setState(set(this.state, ['messages', roomId, messageId], payload))
      // Update cursor if the message was read
      if (roomId === this.state.room.id) {
        const cursor = this.state.user.readCursor({ roomId }) || {}
        const cursorPosition = cursor.position || 0
        cursorPosition < messageId && this.actions.setCursor(roomId, messageId)
        this.actions.scrollToEnd()
      }
      // Send notification
      this.actions.showNotification(payload)
    },

    scrollToEnd: e =>
      setTimeout(() => {
        const elem = document.querySelector('#messages')
        elem && (elem.scrollTop = 100000)
      }, 0),

  }


  componentDidMount() {
    this.makeHolochainCall('holo-chat/chat/register', {}, result => {
      this.actions.setUser({ id: result.Ok })
      console.log(result)
    })
  }

  makeHolochainCall(callString, params, callback) {
    this.state.holochainConnection.then(({call}) => {
      call(callString)(params).then((result) => callback(JSON.parse(result)))
    })
  }

  render() {
    const {
      user,
      room,
      rooms,
      messages,
      sidebarOpen,
      userListOpen,
    } = this.state
    const { createRoom, createConvo } = this.actions

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
                  messages={messages[room.id]}
                />
                <CreateMessageForm state={this.state} actions={this.actions} />
              </col->
              {userListOpen && (
                <UserList
                  room={room}
                  current={user.id}
                  createConvo={createConvo}
                />
              )}
            </row->
          ) : user.id ? (
            <JoinRoomScreen />
          ) : (
            <WelcomeScreen message="Connecting to Holochain..." />
          )}
        </section>
      </main>
    )
  }
}

ReactDOM.render(<View />, document.querySelector('#root'))
