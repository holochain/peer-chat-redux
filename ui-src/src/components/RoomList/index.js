import React from 'react'
import style from './index.module.css'

const Icon = id => (
  <svg>
    <use xlinkHref={`index.svg#${id}`} />
  </svg>
)

export const RoomList = ({
  rooms = [],
  user,
  messages,
  current,
  actions
}) => {

  let refresh

  if(user.id) {
    refresh = (
    <li onClick={actions.getRooms}>
      <input type='image' alt='refresh' src={`refresh.svg`} />
    </li>)
  } else {
    refresh = <li></li>
  }

  return (<ul className={style.component}>
    {rooms.map(room => {
      const messageKeys = Object.keys(messages[room.id] || {})
      const latestMessage =
        messageKeys.length > 0 && messages[room.id][messageKeys.pop()]
      const firstUser = room.users.find(x => x.id !== user.id)
      return (
        <li
          key={room.id}
          disabled={room.id === current.id}
          onClick={e => actions.joinRoom(room)}
        >
          {room.name.match(user.id) && firstUser ? (
            <img src={firstUser.avatarURL} alt={firstUser.id} />
          ) : (
            Icon(room.isPrivate ? 'lock' : 'public')
          )}
          <col->
            <p>{room.name.replace(user.id, '')}</p>
            <span>{latestMessage && latestMessage.text}</span>
          </col->
        </li>
      )
    })}
    {refresh}
  </ul>)
}
