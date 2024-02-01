import { contract } from '../index.js'
import timers from 'node:timers/promises'

const watch = process.argv.includes('--watch')

while (true) {
  const transfersLeft = await contract.participantCountScheduledForTransfer()
  console.log('Transfers left:', transfersLeft)
  if (!watch) break
  await timers.setTimeout(10_000)
}

