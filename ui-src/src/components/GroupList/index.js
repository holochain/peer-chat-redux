import React from 'react'
import style from './index.module.css'

// const Icon = id => (
//   <svg>
//     <use xlinkHref={`index.svg#${id}`} />
//   </svg>
// )

const Emoji = code => (
  String.fromCodePoint(code)
)

export const GroupList = ({
  groups,
  currentGroup,
  joinGroup
}) => {

  return (<ul className={style.component}>
    {groups.map(group => {
      return (
        <li
          key={group.id}
          disabled={group.id === currentGroup.id}
          onClick={e => joinGroup(group)}
        >
          <row->
            {Emoji(group.icon)}
          </row->
        </li>
      )
    })}
  </ul>)
}
