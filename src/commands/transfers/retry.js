/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class RetryCommand extends Command {
  async run() {
    const {argv} = this.parse(RetryCommand)
    const {flags} = this.parse(RetryCommand)

    let transferID = argv[0]

    // Check for auth
    await requireAuth(flags.profile)

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

RetryCommand.args = [
  {
    name: 'TransferID',
    required: true,
    description: 'ID of the transfer to retry.',
  },
]

RetryCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

module.exports = RetryCommand
