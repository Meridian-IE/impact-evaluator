import { createContract } from '../index.js'

export const roundReward = async opts => {
  const { contract } = await createContract(opts)
  console.log((await contract.roundReward()).toString())
}
