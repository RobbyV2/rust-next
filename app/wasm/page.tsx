'use client'

import { useState, useEffect } from 'react'
import { getBasePath } from '../lib/basePath'

type WasmModule = {
  greet: (name: string) => string
  add: (a: number, b: number) => number
  default(path?: string): Promise<void>
}

export default function WasmPage() {
  const [wasm, setWasm] = useState<WasmModule | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [greeting, setGreeting] = useState<string>('')
  const [sum, setSum] = useState<number | null>(null)

  useEffect(() => {
    async function loadWasm() {
      try {
        const basePath = getBasePath()
        const wasmModule = (await import(
          /* webpackIgnore: true */ `${basePath}/wasm/rust_next_wasm.js`
        )) as unknown as WasmModule

        await wasmModule.default(`${basePath}/wasm/rust_next_wasm_bg.wasm`)
        setWasm(wasmModule)
      } catch (err) {
        console.error('WASM load error:', err)
        setError(`Failed to load WASM: ${err}`)
      } finally {
        setLoading(false)
      }
    }

    loadWasm()
  }, [])

  const handleGreet = () => {
    if (!wasm) return
    setGreeting(wasm.greet('World'))
  }

  const handleAdd = () => {
    if (!wasm) return
    setSum(wasm.add(40, 2))
  }

  if (loading) {
    return (
      <main className="p-8 font-sans">
        <h1 className="text-2xl font-bold mb-4">Rust WASM Demo</h1>
        <p className="text-gray-600">Loading WASM module...</p>
      </main>
    )
  }

  if (error) {
    return (
      <main className="p-8 font-sans">
        <h1 className="text-2xl font-bold mb-4">Rust WASM Demo</h1>
        <p className="text-red-600 mb-4">{error}</p>
        <p className="text-gray-600">
          Build WASM first:{' '}
          <code className="bg-gray-100 px-2 py-1 rounded text-sm">just src build-wasm</code>
        </p>
      </main>
    )
  }

  return (
    <main className="p-8 font-sans">
      <h1 className="text-2xl font-bold mb-2">Rust WASM Demo</h1>
      <p className="text-gray-600 mb-8">WASM module loaded successfully!</p>

      <div className="flex flex-col gap-6 max-w-md">
        <div className="p-6 border border-gray-200 rounded-lg">
          <h2 className="text-lg font-semibold mb-4">Greet Function</h2>
          <button
            onClick={handleGreet}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition"
          >
            Call greet(&quot;World&quot;)
          </button>
          {greeting && <p className="mt-3 text-green-600 font-medium">Result: {greeting}</p>}
        </div>

        <div className="p-6 border border-gray-200 rounded-lg">
          <h2 className="text-lg font-semibold mb-4">Add Function</h2>
          <button
            onClick={handleAdd}
            className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition"
          >
            Call add(40, 2)
          </button>
          {sum !== null && <p className="mt-3 text-green-600 font-medium">Result: {sum}</p>}
        </div>
      </div>

      <div className="mt-8">
        <a href="/" className="text-blue-600 hover:underline text-sm">
          &larr; Back to Home
        </a>
      </div>
    </main>
  )
}
