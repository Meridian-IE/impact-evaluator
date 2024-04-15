import { createContract } from '../index.js'

export const currentRoundIndex = async opts => {
  const { contract } = createContract(opts)
  console.log((await contract.currentRoundIndex()).toString())
  process.exit(0)
}
