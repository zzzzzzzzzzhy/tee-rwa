export default function Home() {
  return (
    <main className="min-h-screen flex flex-col items-center justify-center p-8">
      <h1 className="text-4xl font-bold mb-8">TEE-RWA Credit System</h1>
      <p className="text-gray-400 mb-8">
        Privacy-preserving enterprise credit evaluation
      </p>
      <div className="flex gap-4">
        <a
          href="/login"
          className="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg transition"
        >
          Connect Wallet
        </a>
      </div>
    </main>
  )
}
