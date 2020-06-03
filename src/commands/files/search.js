/* eslint-disable new-cap */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const formatBytes = require('../../format-bytes')
const chalk = require('chalk')
const moment = require('moment')

class SearchCommand extends Command {
  async run() {
    const {argv} = this.parse(SearchCommand)
    const query = argv[0]

    // Check for auth
    await requireAuth()

    // Query Put
    await put.Files.Search(query)
    .then(r => {
      // Setup data
      const data = r.data.files

      // Setup columns
      const columns = {
        id: {
          header: 'ID',
        },
        name: {},
        // eslint-disable-next-line camelcase
        file_type: {
          header: 'Type',
        },
        size: {
          header: 'Size',
          get: row => formatBytes(row.size),
        },
        // eslint-disable-next-line camelcase
        updated_at: {
          header: 'Date',
          get: row => moment.utc(row.updated_at).fromNow(),
        },

      }

      // Setup options
      const options = {
        sort: flags.sort,
        filter: flags.filter,
      }

      // Display table
      cli.table(data, columns, options)

      // Friendly display if there's nothing in the list
      if (data.length === 0) {
        this.log(chalk.yellow('No files! :)'))
      }
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

SearchCommand.description = `Search for a file
...
This command allows you search your entire account for a file.
`

SearchCommand.args = [
  {
    name: 'query',
    required: true,
    description: 'Name of item to search for.',
  },
]

module.exports = SearchCommand
