import React from 'react'
import style from './index.module.css'

const placeholder =
  'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7'

export const UserHeader = ({
  state: { user = {} },
  actions: { openFullName }
}) => (
  <header className={style.component}>
    <img src={user.avatarURL ? user.avatarURL : placeholder} alt={user.name ? user.name[0] : ''} onClick={() => openFullName(user.id)}/>
    <div>
      <h3>{user.firstName} {user.lastName} {user.name}</h3>
      <h5>{user.id && `@${user.id.substring(0, 15)}`}</h5>
    </div>
  </header>
)
