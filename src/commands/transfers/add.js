/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
/* eslint-disable no-await-in-loop */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class AddCommand extends Command {
  async run() {
    // Process flags & args
    const {flags} = this.parse(AddCommand)
    const {argv} = this.parse(AddCommand)
    const url = argv[0]
    const folderID = flags.folderID || 0

    // Check for auth
    await requireAuth()

    // Send to put
    cli.action.start('Sending to Put')
    await put.Transfers.Add({url: url, saveTo: folderID})
    .then(r => {
      this.log('Added:', chalk.bold(r.data.transfer.name))
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
    cli.action.stop()
    this.log(chalk.green('All files sent to Put.'))
  }
}

AddCommand.description = `Add a transfer to Put.io
...
Takes a URL or Magnet as an argument and sends it to Put to download.
`

AddCommand.flags = {
  folderID: flags.string({char: 'f', description: 'Folder ID to download into. Defaults to root.'}),
}

AddCommand.args = [
  {
    name: 'URL',
    required: true,
    description: 'URL of the file to download.',
  },
]

module.exports = AddCommand
