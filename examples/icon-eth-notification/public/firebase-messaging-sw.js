importScripts("https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js");
importScripts(
  "https://www.gstatic.com/firebasejs/8.10.0/firebase-messaging.js"
);

const firebaseConfig = {
    apiKey: "AIzaSyBQ6bxRG-BHLIF9W-20Imad4VnMvWEQdPE",
    authDomain: "aurras-15d80.firebaseapp.com",
    projectId: "aurras-15d80",
    storageBucket: "aurras-15d80.appspot.com",
    messagingSenderId: "625844804761",
    appId: "1:625844804761:web:b5c14238120c1873796c08",
    measurementId: "G-1QJ2WE25BE",};

firebase.initializeApp(firebaseConfig);
const messaging = firebase.messaging();

messaging.onBackgroundMessage((payload) => {
  console.log(
    "[firebase-messaging-sw.js] Received background message ",
    payload
  );
  const notificationTitle = payload.notification.title;
  const notificationOptions = {
    body: payload.notification.body,
    icon: icon || '/icons/firebase-logo.png',
    data: restPayload,
  };

  self.registration.showNotification(notificationTitle, notificationOptions);
});
