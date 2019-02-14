import React from 'react'
// import style from './index.module.css'

export const RegistrationForm = ({
  userId,
  registerUser,
}) => (
    <dialog>
      <form
        className={style.component}
        onSubmit={e => {
          e.preventDefault()

          const name = e.target[0].value.trim()
          const avatarUrl = e.target[1].value

          if (name.length > 0 || avatarUrl.length > 0) {
            registerUser({
              id: userId,
              name,
              avatarUrl,
            })
          }
        }}
      >
        <input
          placeholder="Input your handle.."
        />
        <input
          placeholder="Paste avatar image url here.."
        />
        <button type="submit">
          <svg>
            <use xlinkHref="index.svg#send" />
          </svg>
        </button>
      </form>
    </dialog>
)
