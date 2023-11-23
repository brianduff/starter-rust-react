import { Button } from '@blueprintjs/core'
import './App.css'
import { css } from '@emotion/react'

function App() {
  return (
    <>
      <Button>Let's go</Button>
      <div css={css`color: red`}>Hello</div>
    </>
  )
}

export default App
