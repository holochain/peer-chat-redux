import React from 'react'
import style from './index.module.css'
import Linkify from 'react-linkify'

const time = string => {
  const date = new Date(string)
  const minutes = date.getMinutes()
  return `${date.getHours()}:${minutes < 10 ? '0' + minutes : minutes}`
}

export const Message = ({ user }) => message =>
  message.sender ? (
    <li key={message.id} className={style.component}>
      <img
        src={message.sender.avatarURL}
        alt={message.sender.name}
      />
      <div>
        <span>
        {`${message.sender.name} | ${time(message.createdAt)}`}</span>
        <p>
          <Linkify properties={{ target: '_blank' }}>{message.text}</Linkify>
        </p>
      </div>
    </li>
  ) : null
