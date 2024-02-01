import { contractWithSigner } from '../index.js'
import { relative, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const tx = await contractWithSigner.releaseRewards()
console.log(tx.hash)
console.log('Awaiting confirmation...')
await tx.wait()
console.log('Rewards released')

const bin = relative(
  process.cwd(),
  fileURLToPath(new URL('.', import.meta.url))
)
console.log('For progress, check')
console.log()
console.log(`  $ node ${join(bin, 'participant-count-scheduled-for-transfer.js')}`)
console.log()
console.log(`  $ node ${join(bin, 'scheduled-for-transfer.js')}`)
console.log()
process.exit()
