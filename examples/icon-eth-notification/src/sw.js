import { initializeApp } from "firebase/app";
import { getMessaging, onBackgroundMessage } from "firebase/messaging/sw";

const ignored = self.__WB_MANIFEST;

const app = initializeApp({
  apiKey: "AIzaSyBQ6bxRG-BHLIF9W-20Imad4VnMvWEQdPE",
  authDomain: "aurras-15d80.firebaseapp.com",
  projectId: "aurras-15d80",
  storageBucket: "aurras-15d80.appspot.com",
  messagingSenderId: "625844804761",
  appId: "1:625844804761:web:b5c14238120c1873796c08",
  measurementId: "G-1QJ2WE25BE",
});

const messaging = getMessaging(app);

onBackgroundMessage(messaging, (payload) => {
  const { title, body, ...restPayload } = payload.notification;
  console.log(payload.data);
  console.log(payload.notification);

  const notificationOptions = {
    body,
    icon: "/icons/firebase-logo.png",
    data: restPayload,
  };

  return self.registration.showNotification(title, notificationOptions);
});

self.addEventListener("push", (event) => {
  if (event.notification.data && event.notification.data.click_action) {
    self.clients.openWindow(event.notification.data.click_action);
  } else {
    self.clients.openWindow(event.currentTarget.origin);
  }
  event.notification.close();
});
