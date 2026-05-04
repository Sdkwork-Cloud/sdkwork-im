import { App } from './App.js';

const root = document.querySelector('#root');

if (!root) {
  throw new Error('Portal root element "#root" is missing.');
}

App(root);
