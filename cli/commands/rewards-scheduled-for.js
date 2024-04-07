import { createContract } from '../index.js'
import { formatEther } from 'ethers'

export const rewardsScheduledFor = async ({ participant, ...opts }) => {
  const { contract } = await createContract(opts)
  console.log(
    participant,
    formatEther(await contract.rewardsScheduledFor(participant)),
    'FIL'
  )
  process.exit()
}
