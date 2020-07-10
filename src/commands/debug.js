const {Command} = require('@oclif/command')
const config = require('../config')
const chalk = require('chalk')

class ConfigCommand extends Command {
  async run() {
    this.log(chalk.bold(config.path))
    this.log(JSON.stringify(config.store))
  }
}

ConfigCommand.description = `Output the current config
...
This will output the path and current state of the config file used by Kaput-CLI.
Warning: This will include your auth tokens.
`

module.exports = ConfigCommand
