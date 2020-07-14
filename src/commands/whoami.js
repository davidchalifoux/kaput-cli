/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const put = require('../put-api')
const requireAuth = require('../require-auth')
const chalk = require('chalk')

class WhoamiCommand extends Command {
  async run() {
    const {flags} = this.parse(WhoamiCommand)

    // Check for auth
    await requireAuth(flags.profile)

    // Get username
    await put.User.Info()
    .then(r => this.log('Username:', chalk.yellow(r.data.info.username)))
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

WhoamiCommand.description = `Display your username
...
Checks Put.io for the username of the account currently authenticated with the CLI.
`

WhoamiCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

module.exports = WhoamiCommand
