/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command, flags} = require('@oclif/command')
const axios = require('axios').default
const {cli} = require('cli-ux')
const chalk = require('chalk')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')

class IndexersCommand extends Command {
  async run() {
    const {flags} = this.parse(IndexersCommand)
    // Check for auth
    await requireAuth(flags.profile)

    cli.action.start('Talking with chill.institute')

    await axios.get('https://us-central1-kaput-services.cloudfunctions.net/indexersV2', {
      params: {
        token: put.token,
      },
    })
    .then(response => {
      cli.action.stop()
      this.log(chalk.bold.underline('Indexers:'))
      response.data.forEach(element => {
        this.log(element.name)
      })
    })
    .catch(error => {
      this.log(chalk.red(error))
      process.exit(1)
    })
  }
}

IndexersCommand.description = `List indexers
...
Outputs a list of all available indexers that are usable for searching.
`

IndexersCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

module.exports = IndexersCommand
