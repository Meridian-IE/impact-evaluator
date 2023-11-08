import { contractWithSigner } from '../index.js'

const tx = await contractWithSigner.tick()
console.log(tx.hash)
