#!/usr/bin/env node

import yargs from 'yargs/yargs'
import { hideBin } from 'yargs/helpers'
import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

import { currentRoundIndex } from '../commands/current-round-index.js'
import { listTransfers } from '../commands/list-transfers.js'
import {
  participantCountScheduledForTransfer
} from '../commands/participant-count-scheduled-for-transfer.js'
import { readyForTransfer } from '../commands/ready-for-transfer.js'
import { releaseRewards } from '../commands/release-rewards.js'
import { rewardsScheduledFor } from '../commands/rewards-scheduled-for.js'
import { roundReward } from '../commands/round-reward.js'
import { scheduledForTransfer } from '../commands/scheduled-for-transfer.js'
import {
  setMinBalanceForTransfer
} from '../commands/set-min-balance-for-transfer.js'
import { setNextRoundLength } from '../commands/set-next-round-length.js'
import { setRoundReward } from '../commands/set-round-reward.js'
import { tick } from '../commands/tick.js'

const pkg = JSON.parse(
  await fs.readFile(
    fileURLToPath(new URL('../package.json', import.meta.url)),
    'utf8'
  )
)

yargs(hideBin(process.argv))
  .env('MERIDIAN')
  .option('address', {
    alias: 'a',
    type: 'string',
    description: 'Contract address (or $MERIDIAN_ADDRESS)'
  })
  .option('glif-token', {
    alias: 'g',
    type: 'string',
    description: 'GLIF token (or $MERIDIAN_GLIF_TOKEN)'
  })
  .demandOption('address')
  .demandOption('glif-token')
  .command(
    'current-round-index',
    'Get the current round index',
    yargs => yargs,
    currentRoundIndex
  )
  .command(
    'list-transfers',
    'List transfers from last 100 blocks',
    yargs => yargs,
    listTransfers
  )
  .command(
    'participant-count-scheduled-for-transfer',
    'Watch the participant count scheduled for transfer',
    yargs => yargs.option('watch', {
      alias: 'w',
      type: 'boolean',
      default: false
    }),
    participantCountScheduledForTransfer
  )
  .command(
    'ready-for-transfer',
    'List participants ready for transfer',
    yargs => yargs,
    readyForTransfer
  )
  .command(
    'release-rewards',
    'Release rewards',
    yargs => yargs,
    releaseRewards
  )
  .command(
    'rewards-scheduled-for <participant>',
    'Get rewards scheduled for a participant',
    yargs => yargs.positional('participant', { type: 'string' }),
    rewardsScheduledFor
  )
  .command(
    'round-reward',
    'Get the current round reward',
    yargs => yargs,
    roundReward
  )
  .command(
    'scheduled-for-transfer',
    'Get all participants scheduled for transfer',
    yargs => yargs,
    scheduledForTransfer
  )
  .command(
    'set-min-balance-for-transfer <minBalance>',
    'Set the minimum balance for transfer',
    yargs => yargs.positional('minBalance', { type: 'string' }),
    setMinBalanceForTransfer
  )
  .command(
    'set-next-round-length <blocks>',
    'Set the next round length',
    yargs => yargs.positional('blocks', { type: 'number' }),
    setNextRoundLength
  )
  .command(
    'set-round-reward <reward>',
    'Set the round reward',
    yargs => yargs.positional('reward', { type: 'string' }),
    setRoundReward
  )
  .command(
    'tick',
    'Trigger a tick',
    yargs => yargs,
    tick
  )
  .demandCommand()
  .version(`${pkg.name}: ${pkg.version}`)
  .alias('v', 'version')
  .alias('h', 'help')
  .example([
    ['$ $0 current-round-index', 'Get the current round index']
  ])
  .help()
  .parse()
