const {Command, flags} = require('@oclif/command')
const config = require('../config')
const chalk = require('chalk')

class LogoutCommand extends Command {
  async run() {
    const {flags} = this.parse(LogoutCommand)
    const profileName = flags.profile || 'default'

    config.delete(profileName)
    this.log(chalk.yellow('Account removed from the CLI.'))
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
