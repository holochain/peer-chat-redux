import React from 'react'
import style from './index.module.css'

export const CreateRoomForm = ({ submit }) => (
  <form
    className={style.component}
    onSubmit={e => {
      e.preventDefault()

      const name = e.target[0].value

        if (name.length === 0) {
          return
        }

      submit({
        name,
      })
      e.target[0].value = ''
    }}
  >
    <input placeholder="Create a Room" />
    <button type="submit">
      <svg>
        <use xlinkHref="index.svg#add" />
      </svg>
    </button>
  </form>
)
