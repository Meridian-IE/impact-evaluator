import { contract, contractWithSigner } from '../index.js'
import { ethers } from 'ethers'

const reward = BigInt(process.argv[2])
console.log(`setting round reward to ${ethers.formatEther(reward)} FIL`)
console.log(`current value: ${ethers.formatEther(await contract.roundReward())} FIL`)

const tx = await contractWithSigner.setRoundReward(reward)
console.log(tx.hash)
console.log('Awaiting confirmation...')
await tx.wait()
