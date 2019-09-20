import React from 'react'
import style from './index.module.css'

const Icon = id => (
  <svg>
    <use xlinkHref={`index.svg#${id}`} />
  </svg>
)

export const ConversationList = ({
  conversations = [],
  user,
  messages,
  current,
  getConversations,
  joinConversation
}) => {

  let refresh

  if(user.id) {
    refresh = (
    <li onClick={getConversations}>
      <input id='refresh' type='image' alt='refresh' src={`refresh.svg`} />
    </li>)
  } else {
    refresh = <li></li>
  }

  return (<ul className={style.component}>
    {conversations.map(conversation => {
      const messageKeys = Object.keys(messages[conversation.id] || {})
      const latestMessage =
        messageKeys.length > 0 && messages[conversation.id][messageKeys.pop()]
      const firstUser = conversation.users.find(x => x.id !== user.id)
      return (
        <li
          key={conversation.id}
          disabled={conversation.id === current.id}
          onClick={e => joinConversation(conversation)}
        >
          <col->
            <p>{conversation.name.replace(user.id, '')}</p>
            <span>{latestMessage && latestMessage.text}</span>
          </col->
        </li>
      )
    })}
    {refresh}
  </ul>)
}
