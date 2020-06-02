const config = require('./config')
const PutioAPI = require('@putdotio/api-client').default
const put = new PutioAPI({clientID: '4701'})
if (config.has('accessToken')) {
  put.setToken(config.get('accessToken'))
}
module.exports = put
