import React from 'react'
import style from './index.module.css'

export const UserList = ({ room, current, users }) => (
  <ul className={style.component}>
    {room.users.map(user => (
      <li
        key={user}
      >
        <img src={users[user] ? users[user].avatar_url : ''} />
        <p>{users[user] ? users[user].name : '?'}</p>
      </li>
    ))}
  </ul>
)
