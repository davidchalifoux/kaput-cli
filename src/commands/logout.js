const {Command, flags} = require('@oclif/command')
const config = require('../config')
const chalk = require('chalk')

class LogoutCommand extends Command {
  async run() {
    const {flags} = this.parse(LogoutCommand)
    const profileName = flags.profile || 'default'

    if (config.has(profileName)) {
      // Profile exists
      config.delete(profileName)
      this.log(chalk.yellow('Account removed from the CLI.'))
    } else {
      this.log(chalk.yellow(profileName, 'account does not exist. Cannot remove.'))
    }
  }
}

LogoutCommand.description = `Logout from Put
...
Removes your account from the CLI.
`

LogoutCommand.flags = {
  profile: flags.string({description: 'Name of the profile to remove. Defaults to the "default" profile.'}),
}

module.exports = LogoutCommand
