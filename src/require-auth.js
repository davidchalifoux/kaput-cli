/* eslint-disable no-console */
/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable new-cap */
const put = require('./put-api')
const chalk = require('chalk')

async function requireAuth() {
  await put.User.Info()
  .catch(() => {
    console.log(chalk.red('Error: You must first login to the CLI using the "login" command.'))
    process.exit(1)
  })
}

module.exports = requireAuth
