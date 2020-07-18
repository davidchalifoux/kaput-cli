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

class IndexCommand extends Command {
  async run() {
    const {flags} = this.parse(IndexCommand)
    const {argv} = this.parse(IndexCommand)
    const keyword = argv[0]
    const folderID = flags.folderID || 0
    const indexer = flags.indexer
    const nastyResults = !flags.nastyResults

    // Check for auth
    await requireAuth(flags.profile)

    // Search
    let searchResults = []
    cli.action.start('Searching chill.institute')

    await axios.get('https://us-central1-kaput-services.cloudfunctions.net/searchV2', {
      params: {
        token: put.token,
        keyword: keyword,
        indexer: indexer,
        filterNastyResults: nastyResults,
      },
    })
    .then(response => {
      response.data.forEach(element => {
        searchResults.push({name: element.title + ' | ' + chalk.green(formatBytes(element.size)) + ' | ' + chalk.yellow(element.source) + ' | ' + moment.utc(element.upload_time).fromNow(), value: element.link})
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

IndexCommand.description = `Search top indexers with chill
...
Indexer searching is kindly provided by https://chill.institute/
This command allows you to search top trackers to add files to your Put account.
`

IndexCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
  folderID: flags.string({
    char: 'f',
    description:
      'ID of the folder it should download to on Put. Defaults to the root folder.',
  }),
  indexer: flags.string({
    char: 'i',
    description:
      'ID of the indexer to search exclusively.',
  }),
  nastyResults: flags.boolean({
    char: 'n',
    default: false,
    description:
      'If passed, chill.institute will not filter out nasty results.',
  }),
}

IndexCommand.args = [
  {
    name: 'keyword',
    required: true,
    description: 'Name of the content to search for.',
  },
]

module.exports = IndexCommand
