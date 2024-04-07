# meridian cli

## Authentication

Get a token from https://auth.node.glif.io/, then pass `GLIF_TOKEN=...` with
every CLI invocation.

## Ledger

Keep your HW ledger device connected and unlocked on the Ethereum app.

## Installation

```bash
$ npm install --global @meridian-ie/cli
```

## API

```bash
$ meridian --help
meridian <command>

Commands:
  meridian current-round-index              Get the current round index
  meridian list-transfers                   List transfers from last 100 blocks
  meridian                                  Watch the participant count
  participant-count-scheduled-for-transfer  scheduled for transfer
  meridian ready-for-transfer               List participants ready for transfer
  meridian release-rewards                  Release rewards
  meridian rewards-scheduled-for            Get rewards scheduled for a
  <participant>                             participant
  meridian round-reward                     Get the current round reward
  meridian scheduled-for-transfer           Get all participants scheduled for
                                            transfer
  meridian set-min-balance-for-transfer     Set the minimum balance for transfer
  <minBalance>
  meridian set-next-round-length <blocks>   Set the next round length
  meridian set-round-reward <reward>        Set the round reward
  meridian tick                             Trigger a tick

Options:
  -a, --address     Contract address (or $MERIDIAN_ADDRESS)  [string] [required]
  -g, --glif-token  GLIF token (or $MERIDIAN_GLIF_TOKEN)     [string] [required]
  -v, --version     Show version number                                [boolean]
  -h, --help        Show help                                          [boolean]

Examples:
  $ meridian current-round-index  Get the current round index
```
