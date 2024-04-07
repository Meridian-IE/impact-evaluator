import { createContract } from '../index.js'
import timers from 'node:timers/promises'

export const participantCountScheduledForTransfer = async ({ watch, ...opts }) => {
  const { contract } = await createContract(opts)
  while (true) {
    const transfersLeft = await contract.participantCountScheduledForTransfer()
    console.log('Transfers left:', transfersLeft)
    if (!watch) break
    await timers.setTimeout(10_000)
  }
}
