import { createContract } from '../index.js'

export const scheduledForTransfer = async opts => {
  const { contract } = await createContract(opts)
  for (let i = 0; ; i++) {
    try {
      console.log((await contract.scheduledForTransfer(i)))
    } catch (err) {
      break
    }
  }
}
