/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable no-console */
const config = require('./config')
const chalk = require('chalk')
const PutioAPI = require('@putdotio/api-client').default
const Put = new PutioAPI({clientID: '4701'})

// Check for env var for profile name
const profileName = process.env.PUTIO_PROFILE || 'default'
// Check for env var auth token
const authtoken = process.env.PUTIO_TOKEN

if (config.has(profileName + '.authToken')) {
  // Profile exists
  Put.setToken(config.get(profileName + '.authToken'))
} else if (authtoken) {
  Put.setToken(authtoken)
} else {
  // Auth token not provided and profile does not exist
  console.log(chalk.red('Profile', '"' + profileName + '"', 'does not exist.'))
  process.exit(1)
}
module.exports = Put
