import React from 'react'
import '../../index.css'
import { GroupList } from '../GroupList'
import { UserHeader } from '../UserHeader'
import { UserList } from '../UserList'
import { MessageList } from '../MessageList'
import { CreateMessageForm } from '../CreateMessageForm'
import { ConversationList } from '../ConversationList'
import { ConversationHeader } from '../ConversationHeader'
import { StartConversationForm } from '../StartConversationForm'
import { WelcomeScreen } from '../WelcomeScreen'
import { JoinConversationScreen } from '../JoinConversationScreen'
import { RegisterScreen } from '../RegisterScreen'

export const Group = ({
  groups = [],
  currentGroup = {},
  joinGroup,
  sidebarOpen,
  user = {},
  users = [],
  conversations = [],
  messages = [],
  conversation = {},
  getConversations,
  startConversation,
  joinConversation,
  setSidebar,
  sendMessage,
  runCommand,
  getMessages,
  userListOpen,
  setFullName,
  connected,
  registerUser,
  setUserList
}) => (
  <main>
    <aside data-open={sidebarOpen}>
      <UserHeader user={user} />
      <menu>
        <GroupList groups={groups} currentGroup={currentGroup} joinGroup={joinGroup} />
        <ConversationList
          user={user}
          conversations={conversations}
          messages={messages}
          current={conversation}
          getConversations={getConversations}
          joinConversation={joinConversation}
        />
      </menu>
      {user.id && <StartConversationForm submit={startConversation} currentGroup={currentGroup}/>}
    </aside>
    <section>
      <ConversationHeader
        conversation={conversation}
        user={user}
        sidebarOpen={sidebarOpen}
        userListOpen={false}
        setSidebar={setSidebar}
        setUserList={setUserList}
       />
       {conversation.id ? (
          <row->
            <col->
              <MessageList
                user={user}
                users={users}
                messages={messages[conversation.id]}
              />
              <CreateMessageForm user={user} conversation={conversation} message={''} runCommand={runCommand} sendMessage={sendMessage} getMessages={getMessages}/>
            </col->
            {userListOpen && (
              <UserList conversation={conversation} users={users} setFullName={setFullName} />
            )}
          </row->
        ) : connected ? (
          user.id ? <JoinConversationScreen /> : <RegisterScreen registerUser={registerUser} />
        ) : (
          <WelcomeScreen message='You need to create a profile for each chat group you are part of. Please use Identity Manager to set your handle, avatar and optionally your full name.' />
        )}
    </section>
  </main>
)
