# vpl-relying-party-cli

  * [Getting started](#getting-started)
    * [Background](#background)
    * [Requirements](#getting-started-requirements)
        * [MacOS & Linux](#macos-linux)
    * [Installation](#installation)
  * [Usage](#usage)
    * [Configuration file](#configuration-file)
    * [Service](#service)
    * [Account](#account)
    * [Create Account](#create-account)
    * [Set Authority](#set-authority)
    * [Close Account](#close-account)
  * [Development](#development)
    * [Requirements](#development-requirements)
  
## Getting started

<h3 id="getting-started-requirements">Requirements</h4>

## Background

Exzo's programming model and the definitions of the Exzo terms used in this document are available at:

- https://docs.exzo.network/apps
- https://docs.exzo.network/terminology

#### Linux

<h3 id="linux"></h4>

Install devtools:

```bash
sudo apt install build-essential libcurl4-gnutls-dev libxml2-dev libssl-dev libudev-dev
```

#### MacOS & Linux

<h3 id="macos-linux"></h4>

[Install Rust](https://rustup.rs/)

## Installation

Install the package from the [crates](https://crates.io/crates/libc) through `cargo`:

```bash
$ cargo install vpl-relying-party-cli --git https://github.com/ExzoNetwork/exzo-program-library
```

## Usage
 the addresses of the nodes to execute commands to:
You can use the following list of

- `https://rpc-main-1.exzo.network`,
- `https://rpc-test-1.exzo.network`,
- `https://rpc-dev-1.exzo.network`.

### Configuration

The `vpl-relying-party` configuration is shared with the `exzo` command-line tool.

#### To get current `exzo` command-line tool configuration:

```bash
$ exzo config get

Config File: ${HOME}/.config/exzo/cli/config.yml
RPC URL: https://rpc-main-1.exzo.network/
WebSocket URL: https://rpc-main-1.exzo.network/ (computed)
Keypair Path: ${HOME}/.config/exzo/id.json
```

#### To set Cluster RPC URL

See [Exzo clusters](https://docs.exzo.network/clusters/) for cluster-specific RPC URLs

```bash
$ exzo config set --url https://rpc-main-1.exzo.network
```

#### Default Keypair

See [Keypair conventions](https://docs.exzo.network/cli/conventions#keypair-conventions) for information on how to setup a keypair if you don't already have one.

Keypair File

```bash
$ exzo config set --keypair ${HOME}/new-keypair.json
```

### Service

Get the version of the package — ``vpl-relying-party --version``:

```bash
$ vpl-relying-party --version
vpl-relying-party-cli 0.1.0
```

Get all possible package's commands — ``vpl-relying-party --help``:

```bash
vpl-relying-party-cli 0.1.0
VPL Relying Party Command-line Utility

USAGE:
    vpl-relying-party [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show additional information

OPTIONS:
        --authority <KEYPAIR>    Filepath or URL to a relying-party authority keypair. This authority will be able to
                                 close the account. [default: client keypair]
    -C, --config <PATH>          Configuration file to use [default: /Users/user/.config/exzo/cli/config.yml]
        --fee-payer <KEYPAIR>    Filepath or URL to a fee-payer keypair [default: client keypair]
        --url <URL>              JSON RPC URL for the cluster [default: value from configuration file]

SUBCOMMANDS:
    account           Display information stored in relying-party account
    close-account     Close relying-party account
    create-account    Create a new relying-party account
    help              Prints this message or the help of the given subcommand(s)
    set-authority     Set close authority of the relying-party account
```

### Account

Get `relying-party` account parsed data by its address — ``vpl-relying-party account``:

|       Arguments       |  Type  | Required | Description                          |
| :-------------------: | :----: | :------: | ------------------------------------ |
| relying-party-address | Pubkey | Yes      | Account address to get a parsed data.|
| node-url              | URL    | No       | Node URL to apply a command to.      |

```bash
$ vpl-relying-party account Bu1dW7aAyWXDgHPiLS5ch6KFgzaiwFknzKRkmNrzf8xH

Public Key: Bu1dW7aAyWXDgHPiLS5ch6KFgzaiwFknzKRkmNrzf8xH
-------------------------------------------------------------------
Relying Party Data:
    version: 1
    authority: 9atTpuaX8WoxWr7xDanvMmE41bPWkCLnSM4V4CMTu4Lq
    Related program data:
        name: example-dapp333
        icon_cid: QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u
        domain_name: http://example-dapp.com
        redirect_uri: ["http://example-dapp.com/pay", "http://example-dapp.com/play"]
```

### Create Account

Create `relying-party` account — ``vpl-relying-party create-account``:

|       Arguments        |   Type  | Required | Description                                                                                                                                    |
| :--------------------: | :----:  | :------: | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| program-name           | String  | Yes      | The display name associated with relying-party account.                                                                                        |
| program-icon-cid       | String  | Yes      | Content identifier of the icon associated with relying-party account: https://docs.ipfs.io/concepts/content-addressing/                |
| program-domain-name    | URL     | Yes      | Domain name associated with relying-party account.                                                                                            |
| program-redirect-uris  | URIs    | Yes      | Allowed URIs for end-user to be redirected to.                                                                                                |
| lamports               | Number  | No       | Lamports to create a new relying-party account.                                                                                                |
| authority              | Keypair | No       | Filepath or URL to a relying-party authority keypair. This authority will be able to close the account. [default: client keypair]            |
| fee-payer              | Keypair | No       | Filepath or URL to a fee-payer keypair [default: client keypair]                                                                               |

```bash
$ vpl-relying-party create-account "example-dapp333" QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u http://example-dapp.com http://example-dapp.com/pay http://example-dapp.com/play  --url http://127.0.0.1:8899                                                                                                          

Signature: 4CH13f432ygibj2kBsoRJpG4HsnhDRA4pMgrd384bRNS37p5HYEtKQ4mR9iEviiugCMqoBUWa6BLHnEW4CzibatV
Relying Party Address: Bu1dW7aAyWXDgHPiLS5ch6KFgzaiwFknzKRkmNrzf8xH
Created!
```

### Set Authority

Set close authority of the relying-party account — ``vpl-relying-party set-authority``:

| Arguments             | Type    | Required | Description                                                                                                                                    |
| :-------:             | :-----: | :------: | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| relying-party-address | Pubkey  | Yes      | Address of the relying-party account.                                                                                                      |
| new-authority         | Pubkey  | Yes      | New authority of the relying-party account.                                                                                                |
| authority             | Keypair | No       | Filepath or URL to current relying-party authority keypair. [default: client keypair]            |
| fee-payer             | Keypair | No       | Filepath or URL to a fee-payer keypair [default: client keypair]                                                                               |

```bash
$ vpl-relying-party set-authority Bu1dW7aAyWXDgHPiLS5ch6KFgzaiwFknzKRkmNrzf8xH V9u7QR3Ff6mUMdyjvJ67M23Gr6ERzVenuyhXqcc7hFk  --url http://127.0.0.1:8899  

Signature: 4GTJMcrAbQEsqVNz7G4UUUrnWctL6h55fefdwkJGapKrAZyjzeaLusSTS2Kwjp8AnMaEBU7TBeoiUHRJQs9GoQ8S
Authority changed from 9atTpuaX8WoxWr7xDanvMmE41bPWkCLnSM4V4CMTu4Lq to V9u7QR3Ff6mUMdyjvJ67M23Gr6ERzVenuyhXqcc7hFk successful!     
```

### Close Account

Close relying-party account — ``vpl-relying-party close-account``:

| Arguments             | Type    | Required | Description                                                                                                                                    |
| :-------:             | :-----: | :------: | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| relying-party-address | Pubkey  | Yes      | The address of the relying-party account.                                                                                                      |
| receiver-address      | Pubkey  | Yes      | The address to send lamports from relying-party.                                                                                               |
| authority             | Keypair | No       | Filepath or URL to current relying-party authority keypair. [default: client keypair]            |
| fee-payer             | Keypair | No       | Filepath or URL to a fee-payer keypair [default: client keypair]                                                                               |

```bash
$ vpl-relying-party close-account Bu1dW7aAyWXDgHPiLS5ch6KFgzaiwFknzKRkmNrzf8xH V9u7QR3Ff6mUMdyjvJ67M23Gr6ERzVenuyhXqcc7hFk --authority V9u7QR3Ff6mUMdyjvJ67M23Gr6ERzVenuyhXqcc7hFk.json

Signature: 2SFk6rASKYD2Nv8CiiG3TVJavpYj6FRA2xMtWTjSYXwMDEN5RMbA9wEv481DbwZGNeWeLgqmjGZrgESJ99qTxpUJ
Relying-party account closed successful!
```

## Development

<h3 id="development-requirements">Requirements</h4>

- Rust — https://rustup.rs/. Install it with the [following reference](https://rustup.rs/).
- Exzo-CLI — https://docs.exzo.network/cli. Install it with the [following reference](https://docs.exzo.network/cli/install-exzo-cli-tools).
