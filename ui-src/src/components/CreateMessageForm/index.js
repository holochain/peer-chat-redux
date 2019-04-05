import React from 'react'
import style from './index.module.css'

export const CreateMessageForm = ({
  state: { user = {}, room = {}, message = '' },
  actions: { sendMessage, runCommand, getMessages }
}) =>
  room.id ? (
    <form
      className={style.component}
      onSubmit={e => {
        e.preventDefault()

        const message = e.target[0].value.trim()

        if (message.length === 0) {
          return
        }

        e.target[0].value = ''

        message.startsWith('/')
          ? runCommand(message.slice(1))
          : sendMessage({
            text: message,
            roomId: room.id
          })
      }}
    >
      <input
        placeholder='Type a Message..'
      />
      <button type='submit'>
        <svg>
          <use xlinkHref='index.svg#send' />
        </svg>
      </button>
      <button type='submit' onClick={() => getMessages(room.id)}>
        <img src='refresh.svg' alt=''/>
      </button>
    </form>
  ) : null
