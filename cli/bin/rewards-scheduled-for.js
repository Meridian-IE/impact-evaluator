import { contract } from '../index.js'
import { formatEther } from 'ethers'

const [,, address] = process.argv.slice()
console.log(formatEther(await contract.rewardsScheduledFor(address)), 'FIL')
