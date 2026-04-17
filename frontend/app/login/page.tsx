'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import { connectWallet } from '@/lib/api'

export default function LoginPage() {
  const { publicKey, connected } = useWallet()
  const router = useRouter()
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    async function handleConnect() {
      if (publicKey && connected) {
        setLoading(true)
        try {
          await connectWallet(publicKey.toBase58())
          router.push('/dashboard')
        } catch (err) {
          console.error('Failed to connect:', err)
        } finally {
          setLoading(false)
        }
      }
    }
    handleConnect()
  }, [publicKey, connected, router])

  return (
    <main className="min-h-screen flex flex-col items-center justify-center p-8">
      <h1 className="text-3xl font-bold mb-8">Connect Your Wallet</h1>
      
      {loading ? (
        <p className="text-gray-400">Connecting...</p>
      ) : (
        <WalletMultiButton className="!bg-blue-600 hover:!bg-blue-700" />
      )}
      
      <p className="text-gray-500 mt-4 text-sm">
        Connect your Solana wallet to continue
      </p>
    </main>
  )
}
