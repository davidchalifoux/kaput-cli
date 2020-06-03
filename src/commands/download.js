/* eslint-disable unicorn/no-process-exit */
/* eslint-disable no-process-exit */
/* eslint-disable no-await-in-loop */
/* eslint-disable new-cap */
const {Command, flags} = require('@oclif/command')
const {cli} = require('cli-ux')
const put = require('../put-api')
const requireAuth = require('../require-auth')
const formatBytes = require('../format-bytes')
const chalk = require('chalk')
const {DownloaderHelper} = require('node-downloader-helper')

class DownloadCommand extends Command {
  async run() {
    const {flags} = this.parse(DownloadCommand)
    let fileID = flags.fileID || null
    let fileType = null
    let fileURL = null
    let fileName = null

    // Check for auth
    await requireAuth()

    // Get file ID
    while (!fileID) {
      fileID = await cli.prompt('What is the file ID?')
    }

    // Get the file's info
    cli.action.start('Gathering file info')
    await put.File.Get(fileID)
    .then(r => {
      fileType = r.data.file.file_type
      fileName = r.data.file.name
    })
    .catch(error => {
      this.log(chalk.red('Error:', error.data.error_message))
      process.exit(1)
    })

    // FILE: Get storage URl directly
    if (fileType !== 'FOLDER') {
      // Not a folder
      // Get storage URL
      await put.File.GetStorageURL(fileID)
      .then(r => {
        fileURL = r.data.url
      })
      .catch(error => {
        this.log(chalk.red('Error:', error.data.error_message))
        process.exit(1)
      })
      .finally(() => cli.action.stop())
    }

    // ZIP: Create ZIP and get storage URL
    if (fileType === 'FOLDER') {
      cli.action.stop()
      cli.action.start('Creating zip')

      let zipID = null

      await put.Zips.Create({ids: [fileID]})
      .then(r => {
        zipID = r.data.zip_id
      })
      .catch(error => {
        this.log(chalk.red('Error:', error.data.error_message))
        process.exit(1)
      })

      while (fileURL === null) {
        await put.Zips.Get(zipID)
        .then(r => {
          if (r.data.zip_status === 'DONE') {
            fileURL = r.data.url
          }
        })
        .catch(error => {
          this.log(chalk.red('Error:', error.data.error_message))
          process.exit(1)
        })
        await cli.wait()
      }
      cli.action.stop()
    }

    // Start the download
    this.log('')
    this.log(chalk.bold(fileName))
    this.log('')

    const progressBar = cli.progress({
      format: '{bar} | {percentage}% | Progress: {downloadedSize}/{totalSize} | ETA: {eta_formatted} | Speed: {speed}/s',
      barCompleteChar: '\u2588',
      barIncompleteChar: '\u2591',
    })
    const dl = new DownloaderHelper(fileURL, process.cwd(), {
      override: {
        skip: true,
        skipSmaller: false,
      },
    })
    process.on('SIGINT', () => {
      progressBar.stop()
      dl.stop().then(() => this.log('Download stopped. File has been removed.'))
    })
    dl.on('start', () => {
      progressBar.start(100, 0, {
        speed: 'N/A',
        downloadedSize: 'N/A',
        totalSize: 'N/A',
      })
    })
    dl.on('progress', stats => {
      progressBar.update(stats.progress, {
        speed: formatBytes(stats.speed),
        downloadedSize: formatBytes(stats.downloaded),
        totalSize: formatBytes(stats.total),
      })
    })
    dl.on('skip', () => {
      progressBar.stop()
      this.log('File already downloaded.')
    })
    dl.on('end', () => {
      progressBar.stop()
      this.log('')
      this.log('Download finished.')
    })
    dl.start()
  }
}

DownloadCommand.description = `Downloads a file from Put.io
...
Download a file from Put using its ID.
If a folder ID is given, a zip is created and that is downloaded instead.
Note: The ID can be found in the URL of the file from Put.io
`

DownloadCommand.flags = {
  fileID: flags.string({char: 'f', description: 'File ID to download'}),
}

module.exports = DownloadCommand
