import { ethers } from 'ethers'
// import { LedgerSigner } from '@ethersproject/hardware-wallets'
import { LedgerSigner } from "@ethers-ext/signer-ledger"
import HIDTransport from "@ledgerhq/hw-transport-node-hid"
import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

const {
  IE_CONTRACT_ADDRESS = '0x811765AccE724cD5582984cb35f5dE02d587CA12',
  RPC_URL = 'https://api.node.glif.io/rpc/v1',
  GLIF_TOKEN // https://auth.node.glif.io/
} = process.env

const fetchRequest = new ethers.FetchRequest(RPC_URL)
fetchRequest.setHeader('Authorization', `Bearer ${GLIF_TOKEN}`)
const provider = new ethers.JsonRpcProvider(fetchRequest, null, {
  batchMaxCount: 1
})
// provider.on('debug', d => console.log(JSON.stringify(d, null, 2)))
const signer = new LedgerSigner(HIDTransport, provider)
export const contract = new ethers.Contract(
  IE_CONTRACT_ADDRESS,
  JSON.parse(
    await fs.readFile(
      fileURLToPath(new URL('../Abi.json', import.meta.url)),
      'utf8'
    )
  ),
  provider
)
export const contractWithSigner = contract.connect(signer)
