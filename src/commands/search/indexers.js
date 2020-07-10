/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
const {Command} = require('@oclif/command')
const axios = require('axios').default
const {cli} = require('cli-ux')
const chalk = require('chalk')

class IndexersCommand extends Command {
  async run() {
    cli.action.start('Talking with chill.institute')

    await axios.get('https://us-central1-kaput-services.cloudfunctions.net/indexers')
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

module.exports = IndexersCommand
