import { contract, contractWithSigner } from '../index.js'

const tx = await contractWithSigner.releaseRewards()
console.log(tx.hash)
console.log('Awaiting confirmation...')
await tx.wait()
console.log('Rewards released')
console.log('Awaiting transfers...')
while (true) {
  const transfersLeft = await contract.participantCountScheduledForTransfer()
  console.log('Transfers left:', transfersLeft)
  if (transfersLeft === 0n) break
}
