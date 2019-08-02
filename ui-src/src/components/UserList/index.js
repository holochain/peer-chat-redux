import React from 'react'
import style from './index.module.css'

export const UserList = (
  {
    state: { room = {}, users =[] },
    actions: { setFirstName }
  }) => (
  <ul className={style.component}>
    {room.users.map(user => (
      <li key={user} onClick={() => setFirstName(user)}>
        <img src={users[user] ? users[user].avatar_url : ''} alt={users[user] ? users[user].name[0] : ''} />
        <p>{users[user] ? users[user].name : '?'}&nbsp;</p>
        <p>{users[user] ? users[user].full_name : '?'}</p>
      </li>
    ))}
  </ul>
)
