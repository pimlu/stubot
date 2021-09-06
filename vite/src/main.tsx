import React from 'react';
import { render } from 'react-dom';
import App from './ts/App';
import init from './ts/wasm';
import './index.css';

(async () => {
  await init();
  render(<App />, document.getElementById('app')!);
})();