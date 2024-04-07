import { createContract } from '../index.js'
import { formatEther } from 'ethers'

export const listTransfers = async opts => {
  const { contract } = await createContract(opts)
  console.error('Warning: Only showing transfers from the last 100 blocks')
  const events = await contract.queryFilter(contract.filters.Transfer, -100)
  const keys = new Set()
  let sum = 0n

  for (const event of events) {
    const [to, amount] = event.args
    const { blockNumber } = event
    const key = `${to}:${amount}:${blockNumber}`
    if (!keys.has(key)) {
      console.log(`- ${to}: ${formatEther(amount)}`)
      keys.add(key)
      sum += amount
    }
  }

  console.log(`Total: ${formatEther(sum)} / ${sum} attoFIL`)
}
