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
  state = {
    holochainConnection: connect("ws://localhost:3400"),
    user: {},
    users: {},
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
      this.actions.getMessages(room.id)
      this.actions.getRoomMembers(room.id)
      this.actions.scrollToEnd()
    },

    joinRoom: room => {
      this.actions.setRoom(room)
      this.state.messages[room.id] &&
        this.actions.setCursor(
          room.id,
          Object.keys(this.state.messages[room.id]).pop()
        )
    },

    getRoomMembers: roomId => {
      this.makeHolochainCall('holo-chat/chat/get_members', {
          stream_address: roomId,
        }, (result) => {
          console.log(result)

          const users = result.Ok

          users.forEach(address => {
            this.actions.getUserProfile(address)
          })

          this.setState({
            room: {...this.state.room, users}
          })
        })
    },

    sendMessage: ({text, roomId}) => {
      const message = {
        message_type: 'text',
        timestamp: Math.floor(Date.now() / 1000),
        payload: text,
        meta: ''
      }
      this.makeHolochainCall('holo-chat/chat/post_message', {
        stream_address: roomId,
        message,
      }, (result) => {
        console.log('message posted', result)
        
        setTimeout(() => this.actions.getMessages(roomId), 1000); // hack for now
        this.actions.scrollToEnd()
      })
    },

    getMessages: (roomId) => {
      this.makeHolochainCall('holo-chat/chat/get_messages', { address: roomId }, (result) => {
        console.log(result)

        const roomMessages = result.Ok.map(({address, entry}) => ({
          text: entry.payload,
          sender: entry.author,
          createdAt: entry.timestamp,
          id: address
        }))

        this.setState({
          messages: {...this.state.messages, [roomId]: roomMessages}
        })
      })
    },

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
          name: options.name,
          users: []
        })
        this.actions.getRooms()
      })
    },

    getUserProfile: userId => {
      this.makeHolochainCall('holo-chat/chat/get_member_profile', { agent_address: userId }, (result) => {
        console.log(result)
        this.setState({
          users: {...this.state.users, [userId]: result.Ok}
        })
      })
    },

    addUserToRoom: ({ userId, roomId = this.state.room.id }) =>
      this.state.user
        .addUserToRoom({ userId, roomId })
        .then(this.actions.setRoom),

    getRooms: () => {
        this.makeHolochainCall('holo-chat/chat/get_all_public_streams', {}, (result) => {
          console.log(result)
          let rooms = result.Ok.map(({address, entry}) => {
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


    // --------------------------------------
    // Cursors
    // --------------------------------------

    setCursor: (roomId, position) => {
      // this.state.user
      //   .setReadCursor({ roomId, position: parseInt(position) })
      //   .then(x => this.forceUpdate())
    },

    // --------------------------------------
    // Messages
    // --------------------------------------

    // addMessage: payload => {
    //   const roomId = payload.room.id
    //   const messageId = payload.id
    //   // Update local message cache with new message
    //   this.setState(set(this.state, ['messages', roomId, messageId], payload))
    //   // Update cursor if the message was read
    //   if (roomId === this.state.room.id) {
    //     const cursor = this.state.user.readCursor({ roomId }) || {}
    //     const cursorPosition = cursor.position || 0
    //     cursorPosition < messageId && this.actions.setCursor(roomId, messageId)
    //     this.actions.scrollToEnd()
    //   }
    //   // Send notification
    //   this.actions.showNotification(payload)
    // },

    scrollToEnd: e =>
      setTimeout(() => {
        const elem = document.querySelector('#messages')
        elem && (elem.scrollTop = 100000)
      }, 0),

  }


  componentDidMount() {

    // this.makeHolochainCall('holo-chat/chat/get_my_member_id', {}, (result) => {
    //   console.log(result)
    // })

    const registrationData = {
      name: "wollum", 
      avatar_url: "https://avatars3.githubusercontent.com/u/6880154?s=460&v=4.jpeg"
    }

    this.makeHolochainCall('holo-chat/chat/register', registrationData, result => {
      console.log(result)
      this.actions.setUser({ id: result.Ok, name: registrationData.name, avatarURL: registrationData.avatar_url })
      this.actions.getRooms()
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
      users,
      room,
      rooms,
      messages,
      sidebarOpen,
      userListOpen,
    } = this.state
    const { createRoom } = this.actions

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
          ) : user.id ? (
            <RegisterScreen />
          ) : (
            <WelcomeScreen message="Connecting to Holochain..." />
          )}
        </section>
      </main>
    )
  }
}

ReactDOM.render(<View />, document.querySelector('#root'))
