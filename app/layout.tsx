export const metadata = {
  title: 'Rust WASM + Next.js',
  description: 'Rust WASM + Next.js template',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
