/* eslint-disable no-console */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {cli} = require('cli-ux')
const put = require('./put-api')
const chalk = require('chalk')

async function requireAuth() {
  // Check for auth
  cli.action.start(chalk.dim('Checking authentication'))
  await put.User.Info()
  .catch(() => {
    console.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
    process.exit(1)
  })
  .finally(() => {
    cli.action.stop()
    console.log('')
  })
}

module.exports = requireAuth
