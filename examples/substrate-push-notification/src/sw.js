importScripts('https://www.gstatic.com/firebasejs/8.9.1/firebase-app.js');
importScripts('https://www.gstatic.com/firebasejs/8.9.1/firebase-messaging.js');

const ignored = self.__WB_MANIFEST;

firebase.initializeApp({
  apiKey: "xxxxxx",
  authDomain: "xxxxxx",
  projectId: "xxxxxx",
  storageBucket: "xxxxxx",
  messagingSenderId: "xxxxxx",
  appId: "xxxxxx",
  measurementId: "xxxxxx"
});

class CustomPushEvent extends Event {
  constructor(data) {
    super('push');

    Object.assign(this, data);
    this.custom = true;
  }
}

self.addEventListener('push', (e) => {
  if (e.custom) return;
  const oldData = e.data;
  const newEvent = new CustomPushEvent({
    data: {
      ehheh: oldData.json(),
      json() {
        const newData = oldData.json();
        newData.data = {
          ...newData.data,
          ...newData.notification,
        };
        delete newData.notification;
        return newData;
      },
    },
    waitUntil: e.waitUntil.bind(e),
  });
  e.stopImmediatePropagation();
  dispatchEvent(newEvent);
});

const messaging = firebase.messaging();

messaging.onBackgroundMessage((payload) => {
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