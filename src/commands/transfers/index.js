/* eslint-disable new-cap */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../../put-api')
const chalk = require('chalk')

class IndexCommand extends Command {
  async run() {
    const {flags} = this.parse(IndexCommand)

    // Check for auth
    cli.action.start('Checking authentication')
    await put.User.Info()
    .catch(() => {
      this.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
      process.exit(1)
    })
    .finally(() => {
      cli.action.stop()
    })

    // Query Put
    await put.Transfers.Query()
    .then(r => {
      // Setup data
      const data = r.data.transfers.reverse()

      // Setup columns
      const columns = {
        id: {
          header: 'ID',
        },
        name: {},
        // eslint-disable-next-line camelcase
        status_message: {
          header: 'Status',
        },
      }

      // Setup options
      const options = {
        sort: flags.sort,
        filter: flags.filter,
      }

      // Display table
      cli.table(data, columns, options)
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })
  }
}

IndexCommand.description = `List transfers
...
Lists current transfers on the account.
`

IndexCommand.flags = {
  sort: flags.string({description: 'property to sort by (prepend ' - ' for descending)'}),
  filter: flags.string({description: 'filter property by partial string matching, ex: name=foo'}),
}

module.exports = IndexCommand
