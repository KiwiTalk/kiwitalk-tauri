import { injectGlobal } from '@emotion/css';
import React from 'react';
import Login from './pages/Login';

function App() {
  injectGlobal`
    html, body {
      height: 100%;
    }
    body {
      margin: 0;
    }
  `

  return (
    <Login />
  );
}

export default App;
