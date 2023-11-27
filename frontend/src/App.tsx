import { Alignment, Button, Navbar } from '@blueprintjs/core'
import './App.css'
import { css } from '@emotion/react'

function App() {
  return (
    <>
      <Navbar fixedToTop={true} >
        <Navbar.Group align={Alignment.LEFT}>
          <Navbar.Heading>Hello</Navbar.Heading>
          <Navbar.Divider />
        </Navbar.Group>
      </Navbar>
      <Button>Let's go</Button>
      <div css={css`color: red`}>Hello</div>
    </>
  )
}

export default App
