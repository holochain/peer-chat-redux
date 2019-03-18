import React from 'react'
import style from './index.module.css'

export const UserList = ({ room, current, users }) => (
  <ul className={style.component}>
    {room.users.map(user => (
      <li
        key={user}
      >
        <img src={users[user] ? users[user].avatar_url : ''} alt={users[user] ? users[user].name[0] : ''} />
        <p>{users[user] ? users[user].name : '?'}</p>
        <h5>&nbsp;| {user && `@${user.substring(0, 15)}`}</h5>
      </li>
    ))}
  </ul>
)
