/* eslint-disable */

import { initializeApp } from "firebase/app";
import { getMessaging, onBackgroundMessage } from "firebase/messaging/sw";

const ignored = self.__WB_MANIFEST;

const app = initializeApp({
  apiKey: "AIzaSyBFKOWwoGtoPXye_S_tWQHVtOzA_mf874o",
  authDomain: "aurras.firebaseapp.com",
  projectId: "aurras",
  storageBucket: "aurras.appspot.com",
  messagingSenderId: "513296427653",
  appId: "1:513296427653:web:b753ed51e3f52588e7d7ec",
  measurementId: "G-555C15PTHS"
});


const messaging = getMessaging(app);

onBackgroundMessage(messaging, (payload) => {
  const { title, body, icon, ...restPayload } = payload.data;

  const notificationOptions = {
    body,
    icon: icon || '/icons/firebase-logo.png',
    data: restPayload,
  };

  return self.registration.showNotification(title, notificationOptions);
});

self.addEventListener('notificationclick', (event) => {
  if (event.notification.data && event.notification.data.click_action) {
    self.clients.openWindow(event.notification.data.click_action);
  } else {
    self.clients.openWindow(event.currentTarget.origin);
  }
  event.notification.close();
});