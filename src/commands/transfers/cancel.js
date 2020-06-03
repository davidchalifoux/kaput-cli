/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class CancelCommand extends Command {
  async run() {
    const {flags} = this.parse(CancelCommand)
    let transferID = flags.transferID || null

    // Check for auth
    await requireAuth()

    // Confirm transfer ID
    while (!transferID) {
      // eslint-disable-next-line no-await-in-loop
      transferID = await cli.prompt('Transfer ID')
    }

    // Retry transfer
    cli.action.start('Cancelling transfer')
    await put.Transfers.Cancel([transferID])
    .then(() => {
      cli.action.stop()
      this.log(chalk.green('Transfer cancelled.'))
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

CancelCommand.description = `Cancel an ongoing transfer.
...
If transfer is in seeding state, stops seeding. Else, removes transfer entry. Does not remove their files.
`

CancelCommand.flags = {
  transferID: flags.string({char: 'i', description: 'ID of transfer to cancel.'}),
}

module.exports = CancelCommand
