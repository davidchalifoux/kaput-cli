/* eslint-disable no-console */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const Put = require('./put-api')
const chalk = require('chalk')

async function requireAuth() {
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
