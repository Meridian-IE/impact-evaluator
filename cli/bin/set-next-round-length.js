import { contract, contractWithSigner } from '../index.js'

const blocks = Number(process.argv[2])
console.log(`setting next round length to ${blocks} blocks`)
console.log(`current value: ${await contract.nextRoundLength()} blocks`)

const tx = await contractWithSigner.setNextRoundLength(blocks)
console.log(tx.hash)
console.log('Awaiting confirmation...')
await tx.wait()
