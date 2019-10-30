import React from 'react'
import style from './index.module.css'

export const ConversationHeader = ({
  conversation,
  user,
  sidebarOpen,
  userListOpen,
  setSidebar,
  setUserList
}) => (
  <header className={style.component}>
    <button onClick={e => setSidebar(!sidebarOpen)}>
      <svg>
        <use xlinkHref='index.svg#menu' />
      </svg>
    </button>
    {conversation.id ? (<h1>{conversation.name.replace(user.id, '')}</h1>) : (<h1>Set your profile</h1>)}
    {conversation.users && (
      <div onClick={e => setUserList(!userListOpen)}>
        <span>{conversation.users.length}</span>
        <svg>
          <use xlinkHref='index.svg#members' />
        </svg>
      </div>
    )}
  </header>
)
