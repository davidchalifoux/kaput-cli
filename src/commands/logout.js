const {Command} = require('@oclif/command')
const config = require('../config')
const chalk = require('chalk')

class LogoutCommand extends Command {
  async run() {
    config.clear()
    this.log(chalk.yellow('Account removed from the CLI.'))
  }
}

LogoutCommand.description = `Logout from Put
...
Removes your account from the CLI.
`

module.exports = LogoutCommand
