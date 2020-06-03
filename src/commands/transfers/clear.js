/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const chalk = require('chalk')

class ClearCommand extends Command {
  async run() {
    // Check for auth
    cli.action.start('Checking authentication')
    await put.User.Info()
    .catch(() => {
      this.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
      process.exit(1)
    })
    .finally(() => {
      cli.action.stop()
    })

    // Clear transfers
    cli.action.start('Clearing transfers')
    await put.Transfers.ClearAll()
    .then(() => {
      cli.action.stop()
      this.log(chalk.green('Transfers cleared.'))
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

ClearCommand.description = `Clears items in transfers list.
...
This command clears all completed items from the tranfers list.
Note: No data will be removed.
`

module.exports = ClearCommand
