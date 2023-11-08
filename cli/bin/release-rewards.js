import { contractWithSigner } from '../index.js'

const tx = await contractWithSigner.releaseRewards()
console.log(tx.hash)
