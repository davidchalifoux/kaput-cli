kaput-cli
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
kaput-cli/0.0.1 win32-x64 node-v12.17.0
$ kaput --help [COMMAND]
USAGE
  $ kaput COMMAND
...
```
<!-- usagestop -->
```sh-session
$ npm install -g kaput-cli
$ kaput COMMAND
running command...
$ kaput (-v|--version|version)
kaput-cli/0.0.1 darwin-x64 node-v12.17.0
$ kaput --help [COMMAND]
USAGE
  $ kaput COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`kaput download`](#kaput-download)
* [`kaput files`](#kaput-files)
* [`kaput files:search`](#kaput-filessearch)
* [`kaput help [COMMAND]`](#kaput-help-command)
* [`kaput login`](#kaput-login)
* [`kaput logout`](#kaput-logout)
* [`kaput rarbg`](#kaput-rarbg)
* [`kaput transfers`](#kaput-transfers)
* [`kaput transfers:add`](#kaput-transfersadd)
* [`kaput transfers:cancel`](#kaput-transferscancel)
* [`kaput transfers:clear`](#kaput-transfersclear)
* [`kaput transfers:retry`](#kaput-transfersretry)
* [`kaput whoami`](#kaput-whoami)

## `kaput download`

Downloads a file from Put.io

```
USAGE
  $ kaput download

OPTIONS
  -f, --fileID=fileID  File ID to download

DESCRIPTION
  ...
  Download a file from Put using its ID.
  If a folder ID is given, a zip is created and that is downloaded instead.
  Note: The ID can be found in the URL of the file from Put.io
```

_See code: [src\commands\download.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\download.js)_

## `kaput files`

List files from Put

```
USAGE
  $ kaput files

OPTIONS
  -i, --folderID=folderID  folderID to display files in
  --filter=filter          filter property by partial string matching, ex: name=foo
  --sort=sort

DESCRIPTION
  ...
  This command lists all of the files in your root folder by default.
```

_See code: [src\commands\files\index.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\files\index.js)_

## `kaput files:search`

Describe the command here

```
USAGE
  $ kaput files:search

OPTIONS
  -n, --name=name  name to print

DESCRIPTION
  ...
  Extra documentation goes here
```

_See code: [src\commands\files\search.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\files\search.js)_

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

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.0.1/src\commands\help.ts)_

## `kaput login`

Login to Put.io

```
USAGE
  $ kaput login

DESCRIPTION
  ...
  Authenticates the CLI with your Put.io account.
```

_See code: [src\commands\login.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\login.js)_

## `kaput logout`

Unauthenticate the CLI from using your Put.io account.

```
USAGE
  $ kaput logout

DESCRIPTION
  ...
  Removes your account from the CLI.
```

_See code: [src\commands\logout.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\logout.js)_

## `kaput rarbg`

Search and add torrents from RARBG

```
USAGE
  $ kaput rarbg

OPTIONS
  -f, --folderID=folderID  ID of the folder it should download to (on Put.io). Defaults to the root folder.
  -q, --query=query        Name of content to search for.

DESCRIPTION
  ...
  Searches RARBG for matching content.
  Once a torrent is selected, it is sent to Put.io as a transfer.
  Note: The RARBG API can be finicky. If a search returns no results you can try again, or try slightly altering your 
  search.
```

_See code: [src\commands\rarbg.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\rarbg.js)_

## `kaput transfers`

List transfers

```
USAGE
  $ kaput transfers

OPTIONS
  --filter=filter  filter property by partial string matching, ex: name=foo
  --sort=sort

DESCRIPTION
  ...
  Lists current transfers on the account.
```

_See code: [src\commands\transfers\index.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\transfers\index.js)_

## `kaput transfers:add`

Add a transfer to Put.io

```
USAGE
  $ kaput transfers:add

OPTIONS
  -f, --folderID=folderID  (Folder ID to download into. Defaults to root.)
  -u, --url=url            (URL of file to download)

DESCRIPTION
  ...
  Takes a URL or Magnet as an argument and sends it to Put to download.
```

_See code: [src\commands\transfers\add.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\transfers\add.js)_

## `kaput transfers:cancel`

Cancel an ongoing transfer.

```
USAGE
  $ kaput transfers:cancel

OPTIONS
  -i, --transferID=transferID  ID of transfer to cancel.

DESCRIPTION
  ...
  If transfer is in seeding state, stops seeding. Else, removes transfer entry. Does not remove their files.
```

_See code: [src\commands\transfers\cancel.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\transfers\cancel.js)_

## `kaput transfers:clear`

Clears items in transfers list.

```
USAGE
  $ kaput transfers:clear

DESCRIPTION
  ...
  This command clears all completed items from the tranfers list.
  Note: No data will be removed.
```

_See code: [src\commands\transfers\clear.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\transfers\clear.js)_

## `kaput transfers:retry`

Retry a failed transfer

```
USAGE
  $ kaput transfers:retry

OPTIONS
  -i, --transferID=transferID  ID of transfer to retry.

DESCRIPTION
  ...
  Tells Put.io to try a transfer again.
```

_See code: [src\commands\transfers\retry.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\transfers\retry.js)_

## `kaput whoami`

What username you are logged into.

```
USAGE
  $ kaput whoami

DESCRIPTION
  ...
  Checks Put.io for the username of the account currently authenticated with the CLI.
```

_See code: [src\commands\whoami.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src\commands\whoami.js)_
<!-- commandsstop -->
* [`kaput download`](#kaput-download)
* [`kaput help [COMMAND]`](#kaput-help-command)
* [`kaput login`](#kaput-login)
* [`kaput logout`](#kaput-logout)
* [`kaput rarbg`](#kaput-rarbg)
* [`kaput whoami`](#kaput-whoami)

## `kaput download`

Downloads a file from Put.io

```
USAGE
  $ kaput download

OPTIONS
  -f, --fileID=fileID  File ID to download

DESCRIPTION
  ...
  Download a file from Put using its ID.
  If a folder ID is given, a zip is created and that is downloaded instead.
  Note: The ID can be found in the URL of the file from Put.io
```

_See code: [src/commands/download.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src/commands/download.js)_

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

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.0.1/src/commands/help.ts)_

## `kaput login`

Login to Put.io

```
USAGE
  $ kaput login

DESCRIPTION
  ...
  Authenticates the CLI with your Put.io account.
```

_See code: [src/commands/login.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src/commands/login.js)_

## `kaput logout`

Unauthenticate the CLI from using your Put.io account.

```
USAGE
  $ kaput logout

DESCRIPTION
  ...
  Removes your account from the CLI.
```

_See code: [src/commands/logout.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src/commands/logout.js)_

## `kaput rarbg`

Search and add torrents from RARBG

```
USAGE
  $ kaput rarbg

OPTIONS
  -f, --folderID=folderID  ID of the folder it should download to (on Put.io). Defaults to the root folder.
  -q, --query=query        Name of content to search for.

DESCRIPTION
  ...
  Searches RARBG for matching content.
  Once a torrent is selected, it is sent to Put.io as a transfer.
  Note: The RARBG API can be finicky. If a search returns no results you can try again, or try slightly altering your 
  search.
```

_See code: [src/commands/rarbg.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src/commands/rarbg.js)_

## `kaput whoami`

What username you are logged into.

```
USAGE
  $ kaput whoami

DESCRIPTION
  ...
  Checks Put.io for the username of the account currently authenticated with the CLI.
```

_See code: [src/commands/whoami.js](https://github.com/davidchalifoux/kaput-cli/blob/v0.0.1/src/commands/whoami.js)_
<!-- commandsstop -->
