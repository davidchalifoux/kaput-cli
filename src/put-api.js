const config = require('./config')
const PutioAPI = require('@putdotio/api-client').default
const Put = new PutioAPI({clientID: '4701'})
if (config.has('accessToken')) {
  Put.setToken(config.get('accessToken'))
}
module.exports = Put
