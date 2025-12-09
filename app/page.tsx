'use client'

import { useState, useEffect } from 'react'

type WasmModule = {
  greet: (name: string) => string
  add: (a: number, b: number) => number
  default(path?: string): Promise<void>
}

export default function Home() {
  const [wasm, setWasm] = useState<WasmModule | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [greeting, setGreeting] = useState<string>('')
  const [sum, setSum] = useState<number | null>(null)

  useEffect(() => {
    async function loadWasm() {
      try {
        const wasmModule = (await import(
          /* webpackIgnore: true */ '/wasm/rust_next_wasm.js'
        )) as unknown as WasmModule

        await wasmModule.default('/wasm/rust_next_wasm_bg.wasm')
        setWasm(wasmModule)
        setLoading(false)
      } catch (err) {
        console.error('WASM load error:', err)
        setError(`Failed to load WASM: ${err}`)
        setLoading(false)
      }
    }

    loadWasm()
  }, [])

  const handleGreet = () => {
    if (wasm) {
      const result = wasm.greet('World')
      setGreeting(result)
    }
  }

  const handleAdd = () => {
    if (wasm) {
      const result = wasm.add(40, 2)
      setSum(result)
    }
  }

  if (loading) {
    return (
      <main style={{ padding: '2rem', fontFamily: 'system-ui, sans-serif' }}>
        <h1>Rust WASM + Next.js</h1>
        <p>Loading WASM module...</p>
      </main>
    )
  }

  if (error) {
    return (
      <main style={{ padding: '2rem', fontFamily: 'system-ui, sans-serif' }}>
        <h1>Rust WASM + Next.js</h1>
        <p style={{ color: 'red' }}>{error}</p>
        <p>
          Make sure to build WASM first:{' '}
          <code>wasm-pack build --target web --out-dir public/wasm</code>
        </p>
      </main>
    )
  }

  return (
    <main style={{ padding: '2rem', fontFamily: 'system-ui, sans-serif' }}>
      <h1>Rust WASM + Next.js</h1>
      <p>WASM module loaded successfully!</p>

      <div style={{ marginTop: '2rem' }}>
        <h2>Test WASM Functions</h2>

        <div style={{ marginTop: '1rem' }}>
          <button onClick={handleGreet} style={{ padding: '0.5rem 1rem', cursor: 'pointer' }}>
            Call greet(&quot;World&quot;)
          </button>
          {greeting && (
            <p style={{ marginTop: '0.5rem', color: 'green' }}>
              Result: <strong>{greeting}</strong>
            </p>
          )}
        </div>

        <div style={{ marginTop: '1rem' }}>
          <button onClick={handleAdd} style={{ padding: '0.5rem 1rem', cursor: 'pointer' }}>
            Call add(40, 2)
          </button>
          {sum !== null && (
            <p style={{ marginTop: '0.5rem', color: 'green' }}>
              Result: <strong>{sum}</strong>
            </p>
          )}
        </div>
      </div>
    </main>
  )
}
