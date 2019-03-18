import React from 'react'
import style from './index.module.css'
import Linkify from 'react-linkify'

const time = timestamp => {
  const date = new Date(timestamp * 1000)
  const minutes = date.getMinutes()
  return `${date.getHours()}:${minutes < 10 ? '0' + minutes : minutes}`
}

export const Message = ({ user, users }) => message => {
  const profile = users[message.sender] || { name: '?', avatar_url: '' }
  return message.sender ? (
    <li key={message.id} className={style.component}>
      <img
        src={profile.avatar_url}
        alt={profile.name[0]}
      />
      <div>
        <span>
          {`${profile.name} | ${time(message.createdAt)}`}</span>
        <p>
          <Linkify properties={{ target: '_blank' }}>{message.text}</Linkify>
        </p>
      </div>
    </li>
  ) : null
}
