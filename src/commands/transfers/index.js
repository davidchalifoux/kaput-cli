/* eslint-disable new-cap */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const requireAuth = require('../../require-auth')
const chalk = require('chalk')

class IndexCommand extends Command {
  async run() {
    const {flags} = this.parse(IndexCommand)

    // Check for auth
    await requireAuth(flags.profile)

    // Query Put
    await put.Transfers.Query()
    .then(r => {
      // Setup data
      const data = r.data.transfers.reverse()

      // Check for JSON flag
      if (flags.json) {
        // Output to JSON
        this.log(JSON.stringify(data))
      } else {
        // Output as table
        // Setup columns
        const columns = {
          id: {
            header: 'ID',
          },
          name: {},
          // eslint-disable-next-line camelcase
          percent_done: {
            header: 'Progress',
            get: row => row.percent_done + '%',
          },
        }

        // Display table
        cli.table(data, columns)

        // Friendly display if there's nothing in the list
        if (data.length === 0) {
          this.log(chalk.yellow('No transfers! :)'))
        }
      }
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

IndexCommand.description = `Manage your Put.io transfers
...
Lists current transfers on the account.
`

IndexCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
  json: flags.boolean({description: 'Output data as pure JSON instead of in a table.'}),
}

module.exports = IndexCommand
