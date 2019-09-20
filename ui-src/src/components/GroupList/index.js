import React from 'react'
import style from './index.module.css'

const Icon = id => (
  <svg>
    <use xlinkHref={`index.svg#${id}`} />
  </svg>
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
            {Icon(group.icon)}
          </row->
        </li>
      )
    })}
  </ul>)
}
