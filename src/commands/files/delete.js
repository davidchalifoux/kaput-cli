/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class DeleteCommand extends Command {
  async run() {
    const {flags} = this.parse(DeleteCommand)
    const {argv} = this.parse(DeleteCommand)
    const fileID = argv[0]

    // Check for auth
    await requireAuth(flags.profile)

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

DeleteCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

DeleteCommand.args = [
  {
    name: 'fileID',
    required: true,
    description: 'ID of the file to delete.',
  },
]

module.exports = DeleteCommand
