/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable no-await-in-loop */
/* eslint-disable new-cap */
const {Command} = require('@oclif/command')
const {cli} = require('cli-ux')
const config = require('../config')
const put = require('../put-api')
const chalk = require('chalk')

class LoginCommand extends Command {
  async run() {
    let authCode = null
    await put.Auth.GetCode('4701')
    .then(r => {
      authCode = r.data.code
      config.set('authCode', authCode)
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })

    this.log('')
    this.log('Your auth code:', chalk.red.bold(authCode))
    this.log('')
    this.log(chalk.bold('Paste your auth code on put.io/link'))
    this.log('')
    await cli.anykey()

    await put.Auth.CheckCodeMatch(authCode)
    .then(r => {
      config.set('accessToken', r.data.oauth_token)
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })

    this.log(chalk.green('Login successful'))
  }
}

LoginCommand.description = `Login to Put.io
...
Authenticates the CLI with your Put.io account.
`

module.exports = LoginCommand
