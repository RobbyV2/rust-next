import './globals.css'

export const metadata = {
  title: 'Rust + Next.js',
  description: 'Full-stack template with Rust backend and Next.js frontend',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>{children}</body>
    </html>
  )
}
