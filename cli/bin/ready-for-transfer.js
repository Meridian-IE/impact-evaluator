import { contract } from '../index.js'
import { formatEther } from 'ethers'

for (let i = 0;; i++) {
  try {
    const address = await contract.readyForTransfer(i)
    const amount = await contract.rewardsScheduledFor(address)
    console.log(`${address} FIL ${formatEther(amount)}`)
  } catch (err) {
    break
  }
}
process.exit()