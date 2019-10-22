import React from 'react'
import style from './index.module.css'

export const StartConversationForm = ({ submit, currentGroup }) => (
  <form
    className={style.component}
    onSubmit={e => {
      e.preventDefault()

      const name = e.target[0].value

      if (name.length === 0) {
        return
      }

      submit({
        name
      })
      e.target[0].value = ''
    }}
  >
  {currentGroup.id === 'peer-chat-public' ? <input id='conversation' placeholder='This channel is ghost only.' /> : <input id='conversation' placeholder='Start a Conversation' />}


    <button id='submit' type='submit' disabled={currentGroup.id === 'peer-chat-public'}>
      <svg>
        <use xlinkHref='index.svg#add' />
      </svg>
    </button>
  </form>
)
