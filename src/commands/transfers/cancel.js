/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class CancelCommand extends Command {
  async run() {
    const {argv} = this.parse(CancelCommand)
    const transferID = argv[0]

    // Check for auth
    await requireAuth()

    // Retry transfer
    cli.action.start('Cancelling transfer')
    await put.Transfers.Cancel([transferID])
    .then(() => {
      cli.action.stop()
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

CancelCommand.description = `Cancel a transfer
...
If transfer is in seeding state, stops seeding. Else, removes transfer entry. Does not remove their files.
`

CancelCommand.args = [
  {
    name: 'TransferID',
    required: true,
    description: '(ID of the transfer to cancel)',
  },
]

module.exports = CancelCommand
