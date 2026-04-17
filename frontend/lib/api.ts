const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

export async function connectWallet(walletAddress: string) {
  const res = await fetch(`${API_BASE}/api/wallet/connect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ wallet_address: walletAddress }),
  })
  return res.json()
}

export async function getTeeQuote() {
  const res = await fetch(`${process.env.NEXT_PUBLIC_TEE_URL || 'http://localhost:8081'}/quote`)
  return res.json()
}

export async function submitEvaluation(encryptedData: string) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_TEE_URL || 'http://localhost:8081'}/evaluate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ encrypted_data: encryptedData }),
  })
  return res.json()
}
