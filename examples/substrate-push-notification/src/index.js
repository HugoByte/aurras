import React from 'react';
import ReactDOM from 'react-dom';
import firebase from "firebase";

import App from './App';

ReactDOM.render(<App />,
  document.getElementById('root')
);

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker
      .register(`${process.env.PUBLIC_URL}/service-worker.js`)
      .then((registration) => {
        firebase.messaging().useServiceWorker(registration);
        console.log('SW registered: ', registration);
      })
      .catch((registrationError) => {
        console.log('SW registration failed: ', registrationError);
      });
  });
}
