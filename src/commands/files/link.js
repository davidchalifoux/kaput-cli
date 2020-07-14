/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class LinkCommand extends Command {
  async run() {
    const {flags} = this.parse(LinkCommand)
    const {argv} = this.parse(LinkCommand)

    const fileID = argv[0]

    // Check for auth
    await requireAuth(flags.profile)

    await put.File.GetStorageURL(fileID)
    .then(r => {
      this.log(chalk.yellow('Download link:'))
      this.log(chalk.bold(r.data.url))
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

LinkCommand.description = `Generate a download link
...
This command generates a fresh download link.
Note: This link will only work on the device it was generated on.
`

LinkCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

LinkCommand.args = [
  {
    name: 'fileID',
    required: true,
    description: 'ID of the file to generate a link for.',
  },
]

module.exports = LinkCommand
