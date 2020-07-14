/* eslint-disable no-console */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const Put = require('./put-api')
const chalk = require('chalk')
const config = require('./config')

async function requireAuth(profileName = 'default') {
  // Check for env var for profile name
  if (process.env.PUTIO_PROFILE) {
    profileName = process.env.PUTIO_PROFILE
  }

  // Check for env var auth token
  const authtoken = process.env.PUTIO_TOKEN

  // Check for profile
  if (config.has(profileName + '.authToken')) {
    // Profile exists
    Put.setToken(config.get(profileName + '.authToken'))
  } else if (authtoken) {
    // Profile does not exist, check for token
    Put.setToken(authtoken)
  } else {
    // Auth token not provided and profile does not exist
    // eslint-disable-next-line no-lonely-if
    if (profileName === 'default') {
      console.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
      console.log('Note: Default profile does not exist and a different profile was not provided.')
      process.exit(1)
    } else {
      console.log(chalk.red('Profile', '"' + profileName + '"', 'does not exist.'))
      process.exit(1)
    }
  }

  await Put.Auth.ValidateToken()
  .then(r => {
    // Check for token validation
    if (r.data.result === false) {
      console.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
      process.exit(1)
    }
  })
  .catch(() => {
    console.log(chalk.red('Error contacting Put for token validation.'))
    process.exit(1)
  })
}

module.exports = requireAuth
