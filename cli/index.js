import { ethers } from 'ethers'
import { LedgerSigner } from '@ethers-ext/signer-ledger'
import HIDTransport from '@ledgerhq/hw-transport-node-hid'
import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

const { RPC_URL = 'https://api.node.glif.io/rpc/v1' } = process.env

const abi = JSON.parse(
  await fs.readFile(
    fileURLToPath(new URL('./Abi.json', import.meta.url)),
    'utf8'
  )
)

export const createContract = ({ address, glifToken }) => {
  const fetchRequest = new ethers.FetchRequest(RPC_URL)
  fetchRequest.setHeader('Authorization', `Bearer ${glifToken}`)
  const provider = new ethers.JsonRpcProvider(fetchRequest, null, {
    batchMaxCount: 1
  })
  // provider.on('debug', d => console.log(JSON.stringify(d, null, 2)))
  const signer = new LedgerSigner(HIDTransport, provider)
  const contract = new ethers.Contract(address, abi, provider)
  return {
    contract,
    contractWithSigner: contract.connect(signer)
  }
}
