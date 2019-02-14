import * as React from 'react'
import { connect } from 'react-redux'
import { Dispatch } from 'redux'
import { withStyles, Theme, StyleRulesCallback } from '@material-ui/core/styles'
import withRoot from '../../../withRoot'

import {
  Grid,
  Paper
} from '@material-ui/core'

import StreamsContainer from '../containers/streamsContainer'
import MessagesContainer from '../containers/messagesContainer'

import {
  Init
} from '../actions'

const styles: StyleRulesCallback = (theme: Theme) => ({
  chat: {
    display: 'flex',
    height: '100%',
    width: '100%',
    backgroundColor: theme.palette.background.paper
  },
  channels: {
    height: '100%',
    width: '100%'
  },
  messages: {
    height: '100%',
    width: '100%'
  }
})

interface OwnProps {
  classes?: any
}

interface State {
  readonly profileDialogOpen: boolean
}

interface StateProps {

}

interface DispatchProps {
  init: typeof Init.sig,
}

type Props = OwnProps & StateProps & DispatchProps

class DesktopChat extends React.Component<Props, State> {

  constructor (props: any) {
    super(props)
  }

  render (): JSX.Element {
    const { classes } = this.props
    return (
      <div className={classes.chat}>
        <Grid container={true} spacing={0} className={classes.chat}>
          <Grid item={true} xs={3} className={classes.channels}>
            <StreamsContainer {...this.props} isMobile={false} title={'Public Channels'} isPublic={true} />
            <StreamsContainer {...this.props} isMobile={false} title={'Direct Messages'} isPublic={false} />
          </Grid>
          <Grid item={true} xs={7} className={classes.messages}>
            <MessagesContainer {...this.props} isMobile={false} />
          </Grid>
          <Grid item={true} xs={2}>
            <Paper/>
          </Grid>
        </Grid>
      </div>
    )
  }

}

const mapStateToProps = () => {
  return {
  }
}

const mapDispatchToProps = (dispatch: Dispatch): DispatchProps => {
  return {
    init: () => dispatch(Init.create({}))
  }
}

export default connect<StateProps, DispatchProps, OwnProps>(
  mapStateToProps,
  mapDispatchToProps
)(withRoot(withStyles(styles)(DesktopChat)))
