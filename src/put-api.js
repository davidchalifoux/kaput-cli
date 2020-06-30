const config = require('./config')
const PutioAPI = require('@putdotio/api-client').default
const Put = new PutioAPI({clientID: '4701'})
if (config.has('default.authToken')) {
  Put.setToken(config.get('default.authToken'))
}
module.exports = Put
