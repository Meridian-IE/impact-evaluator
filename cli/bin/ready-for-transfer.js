import { contract } from '../index.js'

for (let i = 0;; i++) {
  try {
    console.log((await contract.readyForTransfer(i)))
  } catch (err) {
    break
  }
}
