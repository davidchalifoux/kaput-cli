/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const chalk = require('chalk')

class RetryCommand extends Command {
  async run() {
    const {flags} = this.parse(RetryCommand)
    let transferID = flags.transferID || null

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

    // Confirm transfer ID
    while (!transferID) {
      // eslint-disable-next-line no-await-in-loop
      transferID = await cli.prompt('Transfer ID')
    }

    // Retry transfer
    cli.action.start('Retrying transfer')
    await put.Transfers.Retry(transferID)
    .then(() => {
      cli.action.stop()
      this.log(chalk.green('Transfer retried.'))
      this.log('Note: This does not mean that the transfer was successful.')
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

RetryCommand.description = `Retry a failed transfer
...
Tells Put.io to try a transfer again.
`

RetryCommand.flags = {
  transferID: flags.string({char: 'i', description: 'ID of transfer to retry.'}),
}

module.exports = RetryCommand
