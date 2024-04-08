import { createContract } from '../index.js'
import { formatEther } from 'ethers'

export const readyForTransfer = async opts => {
  const { contract } = await createContract(opts)
  for (let i = 0; ; i++) {
    try {
      const address = await contract.readyForTransfer(i)
      const amount = await contract.rewardsScheduledFor(address)
      console.log(`${address} FIL ${formatEther(amount)}`)
    } catch (err) {
      break
    }
  }
  process.exit()
}
