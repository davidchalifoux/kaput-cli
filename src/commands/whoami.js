/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command} = require('@oclif/command')
const put = require('../put-api')
const requireAuth = require('../require-auth')
const chalk = require('chalk')

class WhoamiCommand extends Command {
  async run() {
    // Check for auth
    await requireAuth()

    // Get username
    await put.User.Info()
    .then(r => this.log('Username:', chalk.yellow(r.data.info.username)))
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

WhoamiCommand.description = `What username you are logged into.
...
Checks Put.io for the username of the account currently authenticated with the CLI.
`

module.exports = WhoamiCommand
