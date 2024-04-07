import { createContract } from '../index.js'
import { ethers } from 'ethers'

export const setMinBalanceForTransfer = async ({ minBalance, ...opts }) => {
  const { contract, contractWithSigner } = await createContract(opts)
  console.log(`setting min balance for transfer to ${ethers.formatEther(minBalance)} FIL`)
  console.log(`current value: ${ethers.formatEther(await contract.minBalanceForTransfer())} FIL`)

  const tx = await contractWithSigner.setMinBalanceForTransfer(minBalance)
  console.log(tx.hash)
}
