import { contract } from '../index.js'

console.log((await contract.currentRoundIndex()).toString())
