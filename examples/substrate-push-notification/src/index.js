import React from 'react'
import ReactDOM from 'react-dom'
// import { getMessaging } from "firebase/messaging/sw";
// import config from './config';
import App from './App'

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
)

if ('Notification' in window) {
  Notification.requestPermission().then((permission) => {
    if (permission === 'denied') {
      console.error("permission denied");
    }
  });
}

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker
      .register(`${process.env.PUBLIC_URL}/firebase-messaging-sw.js`)
      .then((registration) => {
        console.log('SW registered: ', registration);
      })
      .catch((registrationError) => {
        console.log('SW registration failed: ', registrationError);
      });
  });
}