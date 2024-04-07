import { createContract } from '../index.js'

export const tick = async opts => {
  const { contractWithSigner } = await createContract(opts)
  const tx = await contractWithSigner.tick()
  console.log(tx.hash)
}
