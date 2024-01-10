import { contract } from '../index.js'
import { formatEther } from 'ethers'

const [,, address] = process.argv
console.log(address, formatEther(await contract.rewardsScheduledFor(address)), 'FIL')
process.exit()
