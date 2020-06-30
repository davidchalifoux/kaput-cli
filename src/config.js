const Conf = require('conf')
const config = new Conf({migrations: {
  '0.0.1': store => {
    store.set('version', '0.0.1')
    // Remove authCodes, they are one time use only
    if (store.has('authCode')) {
      store.delete('authCode')
    }
    // Rename OAuth tokens
    if (store.has('accessToken')) {
      store.set('default.authToken', store.get('accessToken'))
      store.delete('accessToken')
    }
  },
}})

module.exports = config
