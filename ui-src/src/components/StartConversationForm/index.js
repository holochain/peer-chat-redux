import React from 'react'
import style from './index.module.css'

export const StartConversationForm = ({ submit }) => (
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
    <input id='conversation' placeholder='Start a Conversation' />
    <button id='submit' type='submit'>
      <svg>
        <use xlinkHref='index.svg#add' />
      </svg>
    </button>
  </form>
)
