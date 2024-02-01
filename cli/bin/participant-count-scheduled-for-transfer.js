import { contract } from '../index.js'

const transfersLeft = await contract.participantCountScheduledForTransfer()
console.log('Transfers left:', transfersLeft)
process.exit()
