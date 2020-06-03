/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class DeleteCommand extends Command {
  async run() {
    const {argv} = this.parse(DeleteCommand)
    const fileID = argv[0] || null

    // Check for auth
    await requireAuth()

    // Delete file
    cli.action.start('Deleting file')
    await put.Files.Delete([fileID])
    .then(() => {
      cli.action.stop()
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

DeleteCommand.description = `Delete a file
...
This will delete a file or folder from your account.
Note: If you don't have the trash enabled on your account, this data will be unrecoverable.
`
DeleteCommand.args = [
  {
    name: 'fileID',
    required: true,
    description: 'ID of the file to delete',
  },
]

module.exports = DeleteCommand
