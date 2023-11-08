import { ethers } from 'ethers'
// import { LedgerSigner } from '@ethersproject/hardware-wallets'
import { LedgerSigner } from "@ethers-ext/signer-ledger"
import HIDTransport from "@ledgerhq/hw-transport-node-hid"
import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

const {
  IE_CONTRACT_ADDRESS = '0x8c9f415ee86e65ec72d08b05c42cdc40bfecb8e5',
  RPC_URL = 'https://api.node.glif.io/rpc/v1'
} = process.env

const provider = new ethers.JsonRpcProvider(RPC_URL, null, {
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
