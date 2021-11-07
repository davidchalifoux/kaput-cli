![Logo](https://github.com/davidchalifoux/kaput-cli/blob/master/GitHub-header.png)

Kaput-CLI
=========

CLI tools for Put.io

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/kaput-cli.svg)](https://npmjs.org/package/kaput-cli)
[![Downloads/week](https://img.shields.io/npm/dw/kaput-cli.svg)](https://npmjs.org/package/kaput-cli)
[![License](https://img.shields.io/npm/l/kaput-cli.svg)](https://github.com/davidchalifoux/kaput-cli/blob/master/package.json)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->

# Usage
<!-- usage -->
```sh-session
$ npm install -g kaput-cli
$ kaput COMMAND
running command...
$ kaput (-v|--version|version)
kaput-cli/1.1.1 linux-x64 node-v12.16.3
$ kaput --help [COMMAND]
USAGE
  $ kaput COMMAND
...
```
<!-- usagestop -->

# Commands
<!-- commands -->
* [`kaput debug`](#kaput-debug)
* [`kaput download FILEID`](#kaput-download-fileid)
* [`kaput files [FOLDERID]`](#kaput-files-folderid)
* [`kaput files:delete FILEID`](#kaput-filesdelete-fileid)
* [`kaput files:link FILEID`](#kaput-fileslink-fileid)
* [`kaput files:mkdir FOLDERNAME`](#kaput-filesmkdir-foldername)
* [`kaput files:search QUERY`](#kaput-filessearch-query)
* [`kaput help [COMMAND]`](#kaput-help-command)
* [`kaput login`](#kaput-login)
* [`kaput logout`](#kaput-logout)
* [`kaput search KEYWORD`](#kaput-search-keyword)
* [`kaput search:indexers`](#kaput-searchindexers)
* [`kaput search:top`](#kaput-searchtop)
* [`kaput transfers`](#kaput-transfers)
* [`kaput transfers:add URL`](#kaput-transfersadd-url)
* [`kaput transfers:cancel TRANSFERID`](#kaput-transferscancel-transferid)
* [`kaput transfers:clear`](#kaput-transfersclear)
* [`kaput transfers:retry TRANSFERID`](#kaput-transfersretry-transferid)
* [`kaput whoami`](#kaput-whoami)

## `kaput debug`

Output the current config

```
USAGE
  $ kaput debug

DESCRIPTION
  ...
  This will output the path and current state of the config file used by Kaput-CLI.
  Warning: This will include your auth tokens.
```

_See code: [src/commands/debug.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/debug.js)_

## `kaput download FILEID`

Downloads a file from Put.io

```
USAGE
  $ kaput download FILEID

ARGUMENTS
  FILEID  ID of the file to download.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Downloads a file from Put to your local storage.
  If a folder ID is given, a zip is created and that is downloaded instead.
  Note: The ID can be found in the URL of the file from Put.io
```

_See code: [src/commands/download.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/download.js)_

## `kaput files [FOLDERID]`

Manage your Put.io files

```
USAGE
  $ kaput files [FOLDERID]

ARGUMENTS
  FOLDERID  ID of folder to display files in.

OPTIONS
  --all                      All files of the user will be returned.
  --contentType=contentType  Query Put for the specified content type.
  --fileType=fileType        Query Put for the specified file type.
  --json                     Output data as pure JSON instead of in a table.

  --limit=limit              Number of items to return, if -1 is used, all files will be retreived recursively. Default
                             is 1000.

  --profile=profile          Name of the profile to use for authentication. Defaults to the "default" profile.

  --sort=sort                Property to sort by. Properties available: NAME_ASC, NAME_DESC, SIZE_ASC, SIZE_DESC,
                             DATE_ASC, DATE_DESC, MODIFIED_ASC, MODIFIED_DESC.

DESCRIPTION
  ...
  This command lists all of the files in your root folder by default.
```

_See code: [src/commands/files/index.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/files/index.js)_

## `kaput files:delete FILEID`

Delete a file

```
USAGE
  $ kaput files:delete FILEID

ARGUMENTS
  FILEID  ID of the file to delete.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  This will delete a file or folder from your account.
  Note: If you don't have the trash enabled on your account, this data will be unrecoverable.
```

_See code: [src/commands/files/delete.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/files/delete.js)_

## `kaput files:link FILEID`

Generate a download link

```
USAGE
  $ kaput files:link FILEID

ARGUMENTS
  FILEID  ID of the file to generate a link for.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  This command generates a fresh download link.
  Note: This link will only work on the device it was generated on.
```

_See code: [src/commands/files/link.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/files/link.js)_

## `kaput files:mkdir FOLDERNAME`

Create new folder

```
USAGE
  $ kaput files:mkdir FOLDERNAME

ARGUMENTS
  FOLDERNAME  Name of the new folder.

OPTIONS
  -p, --parentID=parentID  ID of the folder to create the new folder in. Defaults to root.
  --profile=profile        Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Creates a new folder with the given name at the specified folder.
```

_See code: [src/commands/files/mkdir.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/files/mkdir.js)_

## `kaput files:search QUERY`

Search for a file

```
USAGE
  $ kaput files:search QUERY

ARGUMENTS
  QUERY  Name of item to search for.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  This command allows you search your entire account for a file.
```

_See code: [src/commands/files/search.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/files/search.js)_

## `kaput help [COMMAND]`

display help for kaput

```
USAGE
  $ kaput help [COMMAND]

ARGUMENTS
  COMMAND  command to show help for

OPTIONS
  --all  see all commands in CLI
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.1.0/src/commands/help.ts)_

## `kaput login`

Login to Put.io

```
USAGE
  $ kaput login

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Authenticates the CLI with your Put.io account.

  Providing a name to the profile flag allows you to save multiple accounts to Kaput for later use.

  The environment variables PUTIO_PROFILE and PUTIO_TOKEN are also available so that you can switch accounts quickly and 
  without having the token stored locally in a file.
  Setting the environment variable PUTIO_PROFILE tells Kaput which saved profile to use.
  Setting the environment variable PUTIO_TOKEN directly gives Kaput a Put auth token to use.
  These variables do not neeed to be used together. It is recommended to set one or the other.
  Note: This stores the access token locally.
```

_See code: [src/commands/login.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/login.js)_

## `kaput logout`

Logout from Put

```
USAGE
  $ kaput logout

OPTIONS
  --profile=profile  Name of the profile to remove. Defaults to the "default" profile.

DESCRIPTION
  ...
  Removes your account from the CLI.
```

_See code: [src/commands/logout.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/logout.js)_

## `kaput search KEYWORD`

Search top indexers with chill

```
USAGE
  $ kaput search KEYWORD

ARGUMENTS
  KEYWORD  Name of the content to search for.

OPTIONS
  -f, --folderID=folderID  ID of the folder it should download to on Put. Defaults to the root folder.
  -i, --indexer=indexer    ID of the indexer to search exclusively.
  -n, --nastyResults       If passed, chill.institute will not filter out nasty results.
  --profile=profile        Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Indexer searching is kindly provided by https://chill.institute/
  This command allows you to search top trackers to add files to your Put account.
```

_See code: [src/commands/search/index.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/search/index.js)_

## `kaput search:indexers`

List indexers

```
USAGE
  $ kaput search:indexers

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Outputs a list of all available indexers that are usable for searching.
```

_See code: [src/commands/search/indexers.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/search/indexers.js)_

## `kaput search:top`

Get top movies from The Pirate Bay.

```
USAGE
  $ kaput search:top

OPTIONS
  -f, --folderID=folderID  ID of the folder it should download to on Put. Defaults to the root folder.
  --profile=profile        Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Returns the top movies from The Pirate Bay.
```

_See code: [src/commands/search/top.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/search/top.js)_

## `kaput transfers`

Manage your Put.io transfers

```
USAGE
  $ kaput transfers

OPTIONS
  --json             Output data as pure JSON instead of in a table.
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Lists current transfers on the account.
```

_See code: [src/commands/transfers/index.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/transfers/index.js)_

## `kaput transfers:add URL`

Add a transfer to Put.io

```
USAGE
  $ kaput transfers:add URL

ARGUMENTS
  URL  URL of the file to download.

OPTIONS
  -f, --folderID=folderID  Folder ID to download into. Defaults to root.
  --profile=profile        Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Takes a URL or Magnet as an argument and sends it to Put to download.
```

_See code: [src/commands/transfers/add.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/transfers/add.js)_

## `kaput transfers:cancel TRANSFERID`

Cancel a transfer

```
USAGE
  $ kaput transfers:cancel TRANSFERID

ARGUMENTS
  TRANSFERID  ID of the transfer to cancel.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  If transfer is in seeding state, stops seeding. Else, removes transfer entry. Does not remove their files.
```

_See code: [src/commands/transfers/cancel.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/transfers/cancel.js)_

## `kaput transfers:clear`

Clear transfer list

```
USAGE
  $ kaput transfers:clear

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  This command clears all completed items from the tranfers list.
  Note: No data will be removed.
```

_See code: [src/commands/transfers/clear.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/transfers/clear.js)_

## `kaput transfers:retry TRANSFERID`

Retry a failed transfer

```
USAGE
  $ kaput transfers:retry TRANSFERID

ARGUMENTS
  TRANSFERID  ID of the transfer to retry.

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Tells Put.io to try a transfer again.
```

_See code: [src/commands/transfers/retry.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/transfers/retry.js)_

## `kaput whoami`

Display your username

```
USAGE
  $ kaput whoami

OPTIONS
  --profile=profile  Name of the profile to use for authentication. Defaults to the "default" profile.

DESCRIPTION
  ...
  Checks Put.io for the username of the account currently authenticated with the CLI.
```

_See code: [src/commands/whoami.js](https://github.com/davidchalifoux/kaput-cli/blob/v1.1.1/src/commands/whoami.js)_
<!-- commandsstop -->
