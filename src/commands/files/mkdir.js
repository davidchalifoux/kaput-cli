/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class MkdirCommand extends Command {
  async run() {
    const {flags} = this.parse(MkdirCommand)
    const {argv} = this.parse(MkdirCommand)
    const folderName = argv[0]

    // Check for auth
    await requireAuth(flags.profile)

    await put.Files.CreateFolder({parentId: flags.parentID, name: folderName})
    .then(() => {
      this.log(chalk.green('Folder', chalk.yellow(folderName), 'successfully created.'))
    })
    .catch(error => {
      this.log(chalk.red(error.data.error_message))
    })
  }
}

MkdirCommand.description = `Create new folder
...
Creates a new folder with the given name at the specified folder.
`

MkdirCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
  parentID: flags.string({char: 'p', default: 0, description: 'ID of the folder to create the new folder in. Defaults to root.'}),
}

MkdirCommand.args = [
  {
    name: 'folderName',
    description: 'Name of the new folder.',
    required: true,
  },
]

module.exports = MkdirCommand
