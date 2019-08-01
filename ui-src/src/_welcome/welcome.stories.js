import React from 'react'
import { storiesOf } from '@storybook/react'
import { View } from '../index'

storiesOf('Welcome', module)
  .add('Home Page', (() => {
    return <View />
  }))
