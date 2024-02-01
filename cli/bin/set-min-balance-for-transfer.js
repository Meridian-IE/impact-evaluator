import { contract, contractWithSigner } from '../index.js'
import { ethers } from 'ethers'

const minBalance = BigInt(process.argv[2])
console.log(`setting min balance for transfer to ${ethers.formatEther(minBalance)} FIL`)
console.log(`current value: ${ethers.formatEther(await contract.minBalanceForTransfer())} FIL`)

const tx = await contractWithSigner.setMinBalanceForTransfer(minBalance)
console.log(tx.hash)
