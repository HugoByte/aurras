import firebase from "firebase";

export const config = {
    apiKey: "xxxxxx",
    authDomain: "xxxxxx",
    projectId: "xxxxxx",
    storageBucket: "xxxxxx",
    messagingSenderId: "xxxxxx",
    appId: "xxxxxx",
    measurementId: "xxxxxx"
}

firebase.initializeApp(config);
export default firebase;