import configCommon from './dev.json';

import firebase from './firebase';

const config = { ...configCommon, firebase};
export default config;