import React from 'react'
import ReactDOM from 'react-dom'
import { connect } from '@holochain/hc-web-client'
import { Group } from './components/Group'
import './index.css'
// --------------------------------------
// Application
// --------------------------------------
const REACT_APP_WEBSOCKET_INTERFACE = process.env.REACT_APP_WEBSOCKET_INTERFACE
const REACT_APP_PERSONAS_URL = process.env.REACT_APP_PERSONAS_URL

export class View extends React.Component {
  constructor (props) {
    super(props)
    if(REACT_APP_WEBSOCKET_INTERFACE){
      this.state = {
        holochainConnection: connect({ url: REACT_APP_WEBSOCKET_INTERFACE }), // Use for debug
        connected: false,
        user: {},
        users: {},
        conversation: {},
        conversations: [],
        messages: {},
        sidebarOpen: false,
        userListOpen: window.innerWidth > 1000,
        profileSpecSourceDna: '',
        group: {
          id: "peer-chat-public",
          name: "Public",
          icon: 'public'
        },
        groups: []
      }
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
        this.actions.getGroups()
        this.actions.getConversations()
      },

      getGroups: () => {
        this.state.holochainConnection.then(({ call }) => {
          call('admin/interface/list')({}).then(result => {
            console.log(result[0].instances)
            let groups = result[0].instances.filter(function(instance) {
              return instance.id.startsWith('peer-chat');
            }).map(function(instance) {
              return {
                id: instance.id,
                name: instance.name,
                icon: 'public'
              }
            })
            this.setState({ groups: groups})
            console.log(this.state.groups)
          })
        })
      },

      setGroup: group => {
        this.setState({ group: group })
        this.actions.getConversations()
        this.actions.scrollToEnd()
      },

      joinGroup: group => {
        console.log('joining group')
        console.log(group)
        this.makeHolochainCall(group.id + '/chat/get_my_member_profile', {}, (result) => {
          console.log(result)
          const profile = result.Ok
          if (profile) {
            console.log('registration user found with profile:', profile)
            this.actions.setUser({ id: profile.address, name: profile.name, avatarURL: profile.avatar_url })
          }
          else {
            const profileSpecSourceDna = result.Err.Internal
            console.log('User has not registered a profile. redirecting to p&p ' + JSON.stringify(profileSpecSourceDna))
            window.location.replace(`${REACT_APP_PERSONAS_URL}/profile/${profileSpecSourceDna}/${encodeURIComponent(window.location.href)}`)
          }
        })
        this.actions.setGroup(group)
      },
      // --------------------------------------
      // Conversation
      // --------------------------------------

      setConversation: conversation => {
        this.setState({ conversation, sidebarOpen: false })
        this.actions.getMessages(conversation.id)
        this.actions.getConversationMembers(conversation.id)
        this.actions.scrollToEnd()
      },

      joinConversation: conversation => {
        console.log('joining conversation')
        this.actions.setConversation(conversation)
        this.makeHolochainCall(this.state.group.id + '/chat/join_conversation', { conversation_address: conversation.id }, (result) => {
          console.log('joined conversation', result)
        })
      },

      getConversationMembers: conversationId => {
        this.makeHolochainCall(this.state.group.id + '/chat/get_members', {
          conversation_address: conversationId
        }, (result) => {
          console.log('retrieved members', result)
          const users = result.Ok
          users.forEach(address => {
            this.actions.getUserProfile(address)
          })
          this.setState({
            conversation: { ...this.state.conversation, users }
          })
        })
      },

      sendMessage: ({ text, conversationId }) => {
        const message = {
          message_type: 'text',
          timestamp: Math.floor(Date.now() / 1000),
          payload: text,
          meta: ''
        }
        this.makeHolochainCall(this.state.group.id + '/chat/post_message', {
          conversation_address: conversationId,
          message
        }, (result) => {
          console.log('message posted', result)
          this.actions.getMessages(conversationId) // hack for now
          this.actions.scrollToEnd()
        })
      },

      getMessages: (conversationId) => {
        this.makeHolochainCall(this.state.group.id + '/chat/get_messages', { address: conversationId }, (result) => {
          console.log('retrieved messages', result)

          const conversationMessages = result.Ok.map(({ address, entry }) => ({
            text: entry.payload,
            sender: entry.author,
            createdAt: entry.timestamp,
            id: address
          }))

          this.setState({
            messages: { ...this.state.messages, [conversationId]: conversationMessages }
          })
        })
      },

      startConversation: options => {
        console.log(this.state.group)
        const conversationSpec = {
          name: options.name,
          description: '',
          initial_members: []
        }
        this.makeHolochainCall(this.state.group.id + '/chat/start_conversation', conversationSpec, (result) => {
          console.log('created conversation', result)
          this.actions.setConversation({
            id: result.Ok,
            name: options.name,
            users: []
          })
          this.actions.getConversations()
        })
      },

      getUserProfile: userId => {
        this.makeHolochainCall(this.state.group.id + '/chat/get_member_profile', { agent_address: userId }, (result) => {
          console.log('retrieved profile', result)
          this.setState({
            users: { ...this.state.users, [userId]: result.Ok }
          })
        })
      },

      setFullName: userId => {
        console.log('Asked for First Name')
        this.makeHolochainCall(this.state.group.id + '/chat/get_full_name', { agent_address: userId }, (result) => {
          let name = result.Ok.body
          let user = this.state.users[userId]
          user.full_name = name
          console.log(user)
          this.setState({
            users: { ...this.state.users, [userId]: user }
          })

          console.log('Asked for First Name', result.Ok.body)
        })
      },

      getConversations: () => {
        this.makeHolochainCall(this.state.group.id + '/chat/get_all_public_conversations', {}, (result) => {
          console.log('retrieved public conversations', result)
          let conversations = result.Ok.map(({ address, entry }) => {
            return {
              id: address,
              private: !entry.public,
              name: entry.name,
              users: []
            }
          })
          this.setState({
            conversations
          })
        })
      },

      registerUser: ({ name, avatarURL }) => {
        this.makeHolochainCall(this.state.group.id + '/chat/register', { name, avatar_url: avatarURL }, result => {
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
      console.log('this.state.group.id ' + this.state.group.id)
      this.state.holochainConnection.then(({ call }) => {
        call('admin/interface/list')({}).then(result => {
          console.log(result[0].instances)
          let groups = result[0].instances.filter(function(instance) {
            return instance.id.startsWith('peer-chat');
          }).map(function(instance) {
            return {
              id: instance.id,
              name: instance.name,
              icon: 'public'
            }
          })
          this.setState({ groups: groups})
          console.log(this.state.groups)
          this.state.holochainConnection.then(({ callZome, call, onSignal }) => {
          console.log('holochainConnection')
          this.setState({ connected: true })
          onSignal((signal) => {
            console.log(JSON.stringify(signal.signal))
            if (signal.signal.name === 'new_message') {
              console.log(JSON.stringify(signal.signal.name))
              const {conversationId} = JSON.parse(signal.signal.arguments)
              console.log(JSON.parse(signal.signal.arguments))
              console.log(conversationId)
              this.actions.getMessages(conversationId)
            } else if (signal.signal.name === 'new_conversation_member') {
              console.log(JSON.stringify(signal.signal.name))
              const {conversationId} = JSON.parse(signal.signal.arguments)
              this.actions.getConversationMembers(conversationId)
            }
          })
          callZome(this.state.group.id, 'chat', 'get_my_member_profile')({}).then((result) => {
            const profile = JSON.parse(result).Ok
            console.log('result:', result)
            if (profile) {
              console.log('registration user found with profile:', profile)
              this.actions.setUser({ id: profile.address, name: profile.name, avatarURL: profile.avatar_url })
            }
            else {
              const profileSpecSourceDna = JSON.parse(result).Err.Internal
              console.log('User has not registered a profile. redirecting to p&p ' + JSON.stringify(profileSpecSourceDna))
              // if(!window.activateHappWindow) {
              window.activateHappWindow('Identity Manager', `/profile/${profileSpecSourceDna}/Peer Chat`)
              // } else {
                // window.location.replace(`${REACT_APP_PERSONAS_URL}/profile/${profileSpecSourceDna}/${encodeURIComponent(window.location.href)}`)
              // }
            }
          })
        })
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
    let props = {
      user: this.state.user,
      users: this.state.users,
      conversations: this.state.conversations,
      sidebarOpen: this.state.sidebarOpen,
      messages: this.state.messages,
      conversation: this.state.conversation,
      userListOpen: this.state.userListOpen,
      groups: this.state.groups,
      getConversations: this.actions.getConversations,
      startConversation: this.actions.startConversation,
      joinConversation: this.actions.joinConversation,
      setSidebar: this.actions.setSidebar,
      sendMessage: this.actions.sendMessage,
      setUserList: this.actions.setUserList,
      setFullName: this.actions.setFullName,
      joinGroup: this.actions.joinGroup
    }
    return (
      <Group {...props} />
    )
  }
}

ReactDOM.render(<View />, document.querySelector('#root'))
