import React from 'react'
import style from './index.module.css'
import { Message } from '../Message'

const emptyList = (
  <div className={style.empty}>
    <span role='img' aria-label='post'>
      📝
    </span>
    <h2>No Messages Yet</h2>
    <p>Be the first to post in this room!</p>
  </div>
)

export const MessageList = ({ messages = [], user, users, createConvo }) => (
  <ul id='messages' className={style.component}>
    {
      messages.length > 0 ? (
        <wrapper->
          {
            messages
              .sort((a, b) => { return b.createdAt - a.createdAt })
              .map(message => Message({ user, createConvo, users })(message))
          }
        </wrapper->
      ) : (
        emptyList
      )
    }
  </ul>
)
