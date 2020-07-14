/* eslint-disable new-cap */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command, flags} = require('@oclif/command')
const axios = require('axios').default
const formatBytes = require('../../format-bytes')
const moment = require('moment')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')
const inquirer = require('inquirer')
const {cli} = require('cli-ux')

class TopCommand extends Command {
  async run() {
    const {flags} = this.parse(TopCommand)
    const folderID = flags.folderID || 0

    // Check for auth
    await requireAuth(flags.profile)

    // Search
    let searchResults = []
    cli.action.start('Searching chill.institute')

    await axios.get('https://us-central1-kaput-services.cloudfunctions.net/topMovies')
    .then(response => {
      response.data.forEach(element => {
        searchResults.push({name: element.title + ' | ' + chalk.green(formatBytes(element.size)) + ' | ' + moment.utc(element.upload_time).fromNow(), value: element.link})
      })
    })
    .catch(error => {
      this.log(chalk.red(error))
      process.exit(1)
    })
    .finally(() => {
      cli.action.stop()
    })

    // Prompt for selection
    let selectedTorrents = null
    await inquirer
    .prompt([{
      name: 'torrent',
      message: 'What should be sent to Put?',
      type: 'checkbox',
      choices: searchResults,
    }])
    .then(answers => {
      selectedTorrents = answers.torrent
    })
    .catch(error => {
      this.log(chalk.red(error))
      process.exit(1)
    })

    // Send to put
    cli.action.start('Sending to Put')
    for (const magnet of selectedTorrents) {
      // eslint-disable-next-line no-await-in-loop
      await put.Transfers.Add({url: magnet, saveTo: folderID})
      .then(r => {
        this.log('Added:', r.data.transfer.name)
      })
      .catch(error => {
        this.log(chalk.red('Error:', error.data.error_message))
      })
    }
    cli.action.stop()
    this.log(chalk.green('All files sent to Put.'))
  }
}

TopCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
  folderID: flags.string({
    char: 'f',
    description:
      'ID of the folder it should download to on Put. Defaults to the root folder.',
  }),
}

TopCommand.description = `Get top movies from The Pirate Bay.
...
Returns the top movies from The Pirate Bay.
`

module.exports = TopCommand
