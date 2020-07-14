/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable no-await-in-loop */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const config = require('../config')
const put = require('../put-api')
const chalk = require('chalk')

class LoginCommand extends Command {
  async run() {
    const {flags} = this.parse(LoginCommand)
    const profileName = flags.profile || 'default'

    let authCode = null
    await put.Auth.GetCode('4701')
    .then(r => {
      authCode = r.data.code
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
      config.set(profileName + '.authToken', r.data.oauth_token)
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

Providing a name to the profile flag allows you to save multiple accounts to Kaput for later use.

The environment variables PUTIO_PROFILE and PUTIO_TOKEN are also available so that you can switch accounts quickly and without having the token stored locally in a file.
Setting the environment variable PUTIO_PROFILE tells Kaput which saved profile to use.
Setting the environment variable PUTIO_TOKEN directly gives Kaput a Put auth token to use.
These variables do not neeed to be used together. It is recommended to set one or the other.
Note: This stores the access token locally.
`

LoginCommand.flags = {
  profile: flags.string({description: 'Name of the profile to use for authentication. Defaults to the "default" profile.'}),
}

module.exports = LoginCommand
