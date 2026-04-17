'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { useRouter } from 'next/navigation'
import { useEffect, useState } from 'react'
import { FileUpload } from '@/components/FileUpload'
import { getTeeQuote, submitEvaluation } from '@/lib/api'

interface EvaluationResult {
  report_hash: number[]
  credit_score: number
  risk_level: number
  signature: number[]
}

export default function DashboardPage() {
  const { publicKey, connected } = useWallet()
  const router = useRouter()
  const [loading, setLoading] = useState(false)
  const [result, setResult] = useState<EvaluationResult | null>(null)
  const [teePubkey, setTeePubkey] = useState<number[] | null>(null)

  useEffect(() => {
    if (!connected) {
      router.push('/login')
    }
  }, [connected, router])

  useEffect(() => {
    async function fetchQuote() {
      try {
        const quote = await getTeeQuote()
        setTeePubkey(quote.tee_pubkey)
      } catch (err) {
        console.error('Failed to get TEE quote:', err)
      }
    }
    fetchQuote()
  }, [])

  const handleUpload = async (files: File[]) => {
    setLoading(true)
    try {
      const fileData = await Promise.all(
        files.map(f => f.text())
      )
      const encryptedData = btoa(fileData.join('|||'))
      
      const evalResult = await submitEvaluation(encryptedData)
      setResult(evalResult)
    } catch (err) {
      console.error('Evaluation failed:', err)
    } finally {
      setLoading(false)
    }
  }

  const riskLabels = ['Very Low', 'Low', 'Medium', 'High', 'Very High']

  return (
    <main className="min-h-screen p-8">
      <div className="max-w-4xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">Credit Evaluation Dashboard</h1>
          <p className="text-gray-400">
            {publicKey?.toBase58().slice(0, 8)}...{publicKey?.toBase58().slice(-8)}
          </p>
        </div>

        {teePubkey && (
          <div className="bg-gray-800 p-4 rounded-lg mb-6">
            <p className="text-sm text-gray-400">TEE Public Key (registered)</p>
            <p className="text-xs font-mono mt-1 break-all">
              {teePubkey.slice(0, 16).map(b => b.toString(16).padStart(2, '0')).join('')}...
            </p>
          </div>
        )}

        <div className="bg-gray-800 p-6 rounded-lg">
          <h2 className="text-xl font-semibold mb-4">Upload Documents</h2>
          <FileUpload onUpload={handleUpload} disabled={loading} />
        </div>

        {result && (
          <div className="mt-6 bg-gray-800 p-6 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Evaluation Result</h2>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <p className="text-gray-400 text-sm">Credit Score</p>
                <p className="text-3xl font-bold text-green-400">{result.credit_score}</p>
              </div>
              <div>
                <p className="text-gray-400 text-sm">Risk Level</p>
                <p className="text-2xl font-semibold text-yellow-400">
                  {riskLabels[result.risk_level]}
                </p>
              </div>
            </div>
            <div className="mt-4">
              <p className="text-gray-400 text-sm">Report Hash</p>
              <p className="text-xs font-mono break-all mt-1">
                {result.report_hash.slice(0, 16).map(b => b.toString(16).padStart(2, '0')).join('')}...
              </p>
            </div>
          </div>
        )}
      </div>
    </main>
  )
}
